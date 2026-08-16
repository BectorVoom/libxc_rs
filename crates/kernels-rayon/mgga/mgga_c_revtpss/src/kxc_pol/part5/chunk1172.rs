//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1172/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1172(t251: f64, t5977: f64, t1558: f64, t1568: f64, t10519: f64, t10539: f64, t14498: f64, t14506: f64, t14511: f64, t14512: f64, t14518: f64, t14522: f64, t14525: f64, t14533: f64, t14539: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t5978: f64, t820: f64, t837: f64) -> (f64, f64, f64) {
    let t18677 = t251 * t5977;
    let t18681 = t1568 * t1558;
    let t18687 = 0.13009920719177044025e-1_f64 * t10519 + t14498 + t14506 + t14511 + 0.13009920719177044025e-2_f64 * t14512 - 0.65854491829355115987e0_f64 * t820 * t2815 * t5978 - t14518 - t14522 - 0.23131639038696784278e-2_f64 * t14525 - 0.13170898365871023197e1_f64 * t4514 * t4494 * t4424 - 0.65854491829355115987e0_f64 * t4514 * t18677 * t837 - 0.13170898365871023197e1_f64 * t4514 * t18681 * t837 - 0.14634331517634470219e-1_f64 * t14533 + t14539 - 0.11565819519348392139e-2_f64 * t10539;
    (t18677, t18681, t18687)
}
