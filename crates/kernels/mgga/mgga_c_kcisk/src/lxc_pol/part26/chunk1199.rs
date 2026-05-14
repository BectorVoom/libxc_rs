//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1199/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1199<F: Float>(t2347: F, t9891: F, t2748: F, t8455: F, t8436: F, t15094: F, t1611: F, t21345: F, t33745: F, t34832: F, t34834: F, t34848: F, t34851: F, t34906: F, t4535: F, t9557: F, t9882: F) -> (F, F, F, F) {
    let t34909 = t9891 * t2347;
    let t34912 = t2748 * t8455;
    let t34919 = t2748 * t8436;
    let t34922 = -6.0 * t15094 * t34919 - t1611 * t34906 + 4.0 * t21345 * t9882 - 2.0 * t2347 * t33745 + 4.0 * t34909 * t4535 + 2.0 * t34912 * t4535 - t8455 * t9557 - t34832 + t34834 - t34848 - t34851;
    (t34909, t34912, t34919, t34922)
}
