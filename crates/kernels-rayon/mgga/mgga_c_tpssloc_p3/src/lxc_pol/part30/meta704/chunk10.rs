//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2307/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2307(t23384: f64, t28638: f64, t23665: f64, t28605: f64, t1610: f64, t17876: f64, t1953: f64, t23346: f64, t23685: f64, t23696: f64, t25706: f64, t28641: f64, t3200: f64, t4615: f64, t4684: f64, t5677: f64, t6687: f64, t7622: f64, t89151: f64, t89156: f64, t89158: f64) -> f64 {
    let t100189 = t23384 * t28638;
    let t100193 = t23665 * t28605;
    let t100195 = 0.36554090374405031923e-2_f64 * t6687 * t23696 * t23685 * t5677 - t3200 * t28641 * t4684 + t17876 * t1953 + 2.0_f64 * t1610 * t25706 - 0.97477574331746751795e-2_f64 * t23346 * t28638 + 0.12184696791468343974e-2_f64 * t100189 + t89151 + 2.0_f64 * t4615 * t7622 - 0.54831135561607547883e-2_f64 * t100193 + t89156 + t89158;
    t100195
}
