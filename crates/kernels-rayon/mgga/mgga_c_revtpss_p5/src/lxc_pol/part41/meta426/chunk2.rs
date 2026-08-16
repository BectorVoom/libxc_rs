//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1488/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1488(t116912: f64, t31538: f64, t105880: f64, t116946: f64, t117450: f64, t117457: f64, t117460: f64, t117462: f64, t117470: f64, t117473: f64, t117482: f64, t117484: f64, t117497: f64, t117510: f64, t21850: f64, t21876: f64, t31035: f64, t31039: f64, t31058: f64, t5823: f64, t5891: f64, t5895: f64, t5915: f64, t658: f64, t665: f64, t8258: f64, t8259: f64, t8267: f64, t8268: f64) -> f64 {
    let t118348 = t116912 * t31538;
    let t118353 = -t117450 - 110.0_f64 / 27.0_f64 * t117457 - t117460 + 10.0_f64 / 9.0_f64 * t117462 + 44.0_f64 / 9.0_f64 * t117470 + t117473 - t117482 + t117484 + t117497 - 3.0_f64 / 4.0_f64 * t31035 * t8259 * t105880 + 5.0_f64 / 12.0_f64 * t8258 * t8268 * t5915 * t658 + 5.0_f64 / 18.0_f64 * t8258 * t31058 * t5895 * t665 + 5.0_f64 / 108.0_f64 * t8267 * t116946 * t5895 * t658 + 5.0_f64 / 12.0_f64 * t8258 * t8268 * t5823 * t665 - 5.0_f64 / 36.0_f64 * t8267 * t31058 * t5823 * t658 - t117510 - 5.0_f64 / 24.0_f64 * t8267 * t8268 * t21850 + t8258 * t8259 * t21876 / 4.0_f64 + 2.0_f64 * t118348 + 5.0_f64 / 4.0_f64 * t31035 * t31039 * t5891;
    t118353
}
