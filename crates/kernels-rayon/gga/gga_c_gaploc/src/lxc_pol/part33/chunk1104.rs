//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1104/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1104(t15483: f64, t2615: f64, t9438: f64, t7416: f64, t9830: f64, t10029: f64, t2464: f64, t2465: f64, t2684: f64, t7258: f64, t22424: f64, t3311: f64) -> (f64, f64, f64, f64, f64) {
    let t28818 = t2615 * t9438 * t15483;
    let t28820 = t7416 * t9830;
    let t28822 = t7416 * t10029;
    let t28827 = 0.17041300423964777634e0_f64 * t2684 * t2464 * t2465 * t7258;
    let t28828 = t22424 * t3311;
    (t28818, t28820, t28822, t28827, t28828)
}
