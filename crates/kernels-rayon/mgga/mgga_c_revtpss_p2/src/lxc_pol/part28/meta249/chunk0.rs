//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1110/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1110(t4012: f64, t5627: f64, t828: f64, t3826: f64, t187: f64, t5566: f64, t1856: f64, t72: f64, t757: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t5546: f64, t5548: f64, t5568: f64, t5570: f64, t5573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5629 = t4012 * t828 * t5627;
    let t5632 = 0.18311447306006545054e-3_f64 * t3826;
    let t5634 = 0.19751673498613801407e-1_f64 * t5566 * t187;
    let t5635 = t1856 * t72;
    let t5636 = t5635 * t757;
    let t5637 = 0.18311447306006545054e-3_f64 * t5636;
    let t5638 = -t2569 + t2579 + t2587 - t2522 + t5546 - t5548 + t5568 + t5570 - t5573 - t5632 - t2562 + t5634 - t5637;
    (t5629, t5632, t5634, t5635, t5637, t5638)
}
