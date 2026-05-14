//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 817/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk817<F: Float>(t1611: F, t2347: F, t240: F, t2748: F, t4535: F, t555: F, t6604: F, t9557: F, t9828: F, t9829: F, t9830: F, t9833: F, t9849: F, t9878: F, t9882: F, t9891: F) -> (F,) {
    let t9895 = t9828 - t9829 - t9830 + t9833 - t9849 + t240 * (-t1611 * t9891 - t2347 * t9557 - t2748 * t6604 + 2.0 * t4535 * t9882 + t555 * t9878 - t9828 + t9829 + t9830 - t9833 + t9849);
    (t9895,)
}
