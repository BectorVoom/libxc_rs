//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1066/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1066(t3: f64, t3141: f64, t163: f64, t4014: f64, t3997: f64, t732: f64, t166: f64, t736: f64, t169: f64, t6270: f64, t2098: f64, t712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10212 = t3141 * t3;
    let t10218 = t163 * t4014;
    let t10221 = t732 * t3997;
    let t10226 = t166 * t4014;
    let t10229 = t736 * t3997;
    let t10234 = t169 * t4014;
    let t10237 = t6270 * t3997;
    let t10242 = t2098 * t4014;
    let t10245 = t712 * t3997;
    (t10212, t10218, t10221, t10226, t10229, t10234, t10237, t10242, t10245)
}
