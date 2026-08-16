//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3092/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3092(t16784: f64, t6548: f64, t24494: f64, t3531: f64, t1196: f64, t5181: f64, t6555: f64, t20896: f64, t5192: f64, t81352: f64, t81558: f64, t81560: f64, t81562: f64, t81566: f64, t81570: f64, t81573: f64) -> (f64, f64, f64, f64, f64) {
    let t81575 = 0.35089341735807877242e1_f64 * t16784 * t6548;
    let t81577 = 0.10389515463408878255e3_f64 * t3531 * t24494;
    let t81580 = 0.10526802520742363173e2_f64 * t1196 * t6555 * t5181;
    let t81582 = 0.51947577317044391276e2_f64 * t5192 * t20896;
    let t81583 = t81352 + t81558 + t81560 - t81562 + t81566 + t81570 + t81573 + t81575 + t81577 - t81580 - t81582;
    (t81575, t81577, t81580, t81582, t81583)
}
