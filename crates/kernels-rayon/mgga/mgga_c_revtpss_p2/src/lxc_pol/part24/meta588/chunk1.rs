//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1837/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1837(t1437: f64, t22953: f64, t4003: f64, t47442: f64, t47454: f64, t49432: f64, t5735: f64, t5745: f64, t75274: f64, t820: f64, t86634: f64, t86639: f64, t86643: f64, t86647: f64, t86654: f64, t92064: f64) -> f64 {
    let t92409 = t47442 + 0.43902994552903410657e-1_f64 * t75274 - 0.13170898365871023197e0_f64 * t86634 - 0.39029762157531132076e-1_f64 * t86639 + 0.65854491829355115985e-1_f64 * t86643 - 0.13170898365871023197e0_f64 * t86647 + 0.65854491829355115985e-1_f64 * t86654 - 0.65854491829355115987e0_f64 * t820 * t1437 * t92064 - 0.18505311230957427423e-1_f64 * t49432 + t47454 + 0.52683593463484092788e1_f64 * t5745 * t5735 * t4003 * t22953;
    t92409
}
