//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 173/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk173(t576: f64, t577: f64, t11: f64, t2: f64, t10: f64, t3: f64, t9: f64, t16: f64, t15: f64, t14: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t580 = 1.0_f64 + 0.45e1_f64 * t576 * t577;
    let t581 = t2 * t11;
    let t582 = 0.174e1_f64 * t581;
    let t583 = t10 * t3;
    let t584 = 1.0_f64 / t583;
    let t586 = 0.174e1_f64 * t9 * t584;
    let t587 = t9 * t2;
    let t588 = t587 * t16;
    let t589 = 2.0_f64 * t588;
    let t590 = t15 * t3;
    let t591 = 1.0_f64 / t590;
    let t592 = t14 * t591;
    (t580, t581, t582, t583, t584, t586, t587, t588, t589, t590, t591, t592)
}
