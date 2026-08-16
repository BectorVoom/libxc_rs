//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1393/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1393(t1343: f64, t1353: f64, t1448: f64, t1450: f64, t198: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t4139: f64, t532: f64, t5532: f64, t5536: f64, t5537: f64, t5541: f64, t5542: f64, t5546: f64, t5548: f64, t5568: f64, t5570: f64, t5573: f64, t5591: f64, t5632: f64, t5778: f64) -> f64 {
    let t5782 = t1450 * t198 * t532 * t5778 + 3.0_f64 * t1343 * t198 * t5591 + 3.0_f64 * t1353 * t4139 * t5532 + 6.0_f64 * t1353 * t5536 * t5537 - t1448 * t5541 * t5542 - t2522 - t2562 - t2569 + t2579 + t2587 + t5546 - t5548 + t5568 + t5570 - t5573 - t5632;
    t5782
}
