//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1362/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1362<F: Float>(t119752: F, t119793: F, t119832: F, t119872: F, t1459: F, t27060: F, t32229: F, t114849: F, t2282: F, t27064: F, t34843: F, t41209: F, t14287: F, t34849: F, t2732: F, t85381: F) -> (F, F, F, F, F, F, F) {
    let t119875 = t1459 * (t119752 + t119793 + t119832 + t119872);
    let t119877 = 4.0 * t32229 * t27060;
    let t119879 = 2.0 * t114849 * t2282;
    let t119881 = 2.0 * t32229 * t27064;
    let t119883 = 6.0 * t41209 * t34843;
    let t119885 = 2.0 * t14287 * t34849;
    let t119886 = t85381 * t2732;
    (t119875, t119877, t119879, t119881, t119883, t119885, t119886)
}
