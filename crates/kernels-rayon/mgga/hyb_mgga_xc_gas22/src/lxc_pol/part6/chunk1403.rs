//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1403/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1403(t21832: f64, t21836: f64, t21840: f64, t21845: f64, t21850: f64, t21856: f64, t21894: f64, t21932: f64, t25937: f64, t25939: f64, t25941: f64, t25944: f64, t25946: f64, t25948: f64, t25951: f64, t25954: f64, t25957: f64) -> f64 {
    let t30387 = t21832 + 0.97661052298701573622e-3_f64 * t25937 - 0.43374325201206959368e-1_f64 * t25939 + 0.96319466275353142155e0_f64 * t25941 + 0.43374325201206959368e-1_f64 * t25944 + 0.32530743900905219526e-1_f64 * t25946 - 0.65061487801810439052e-1_f64 * t25948 - t21836 - t21840 - t21845 + t21850 + 2.0_f64 * t25951 - t21856 + 4.0_f64 * t25954 + 2.0_f64 * t25957 + t21894 + t21932;
    t30387
}
