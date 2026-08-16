//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1411/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1411(t21754: f64, t606: f64, t4186: f64, t4210: f64, t2282: f64, t5825: f64, t18281: f64, t60: f64, t10379: f64, t1480: f64, t21733: f64, t21736: f64, t21742: f64, t21745: f64, t4211: f64, t4214: f64, t44: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t614: f64, t620: f64) -> f64 {
    let t21755 = t21754 * t606;
    let t21758 = t4210 * t4186;
    let t21761 = t2282 * t5825;
    let t21762 = t21761 * t606;
    let t21765 = t60 * t18281;
    let t21768 = -20.0_f64 / 27.0_f64 * t614 * t5835 - 5.0_f64 / 108.0_f64 * t44 * t21733 + 5.0_f64 / 9.0_f64 * t44 * t21736 - 20.0_f64 / 9.0_f64 * t614 * t5838 + 5.0_f64 / 18.0_f64 * t44 * t21742 + 5.0_f64 / 6.0_f64 * t44 * t21745 - 220.0_f64 / 27.0_f64 * t5843 * t620 - 40.0_f64 / 27.0_f64 * t1480 * t4211 + 40.0_f64 / 9.0_f64 * t1480 * t4214 + 5.0_f64 / 108.0_f64 * t56 * t21755 + 5.0_f64 / 9.0_f64 * t56 * t21758 + 5.0_f64 / 18.0_f64 * t56 * t21762 - 5.0_f64 / 6.0_f64 * t56 * t21765 + t10379;
    t21768
}
