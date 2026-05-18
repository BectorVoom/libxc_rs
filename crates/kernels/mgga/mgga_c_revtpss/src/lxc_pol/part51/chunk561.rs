//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 561/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk561<F: Float>(t1015: F, t4186: F, t1012: F, t3147: F, t72: F, t3088: F, t3299: F, t1668: F, t3153: F, t1043: F, t3154: F, t3117: F) -> (F, F, F, F, F, F, F) {
    let t4886 = t1015 * t4186;
    let t4887 = t1012 * t4886;
    let t4890 = t3147 * t72;
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4893 = t1668 * t3153;
    let t4894 = t3154 * t1043;
    let t4895 = t4893 * t4894;
    let t4896 = t3117 * t4895;
    (t4887, t4890, t4891, t4892, t4893, t4895, t4896)
}
