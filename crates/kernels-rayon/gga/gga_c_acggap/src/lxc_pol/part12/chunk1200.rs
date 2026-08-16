//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1200/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1200(t35587: f64, t35594: f64, t35596: f64, t35608: f64, t35610: f64, t35623: f64, t31421: f64, t31426: f64, t31429: f64, t35580: f64, t35585: f64, t35591: f64, t35599: f64, t35601: f64, t35614: f64, t35616: f64, t35621: f64) -> f64 {
    let t37622 = 0.85748036236139473944e-3_f64 * t35587;
    let t37624 = 0.85748036236139473944e-3_f64 * t35594;
    let t37625 = 0.25724410870841842184e-2_f64 * t35596;
    let t37631 = 0.41930789719472202758e-3_f64 * t35608;
    let t37632 = 0.11321313224257494745e-1_f64 * t35610;
    let t37636 = 0.12579236915841660828e-2_f64 * t35623;
    let t37637 = 0.25158473831683321655e-2_f64 * t35580 - 0.5031694766336664331e-2_f64 * t35585 + t37622 - 0.64311027177104605458e-2_f64 * t35591 + t37624 + t37625 + 0.12862205435420921092e-1_f64 * t35599 + 0.11321313224257494744e0_f64 * t35601 + 0.4584375e-1_f64 * t31421 - 0.16809375e0_f64 * t31426 - 11.0_f64 / 48.0_f64 * t31429 + t37631 + t37632 - 0.94344276868812456207e-3_f64 * t35614 - 0.31448092289604152068e-2_f64 * t35616 - 0.15724046144802076034e-2_f64 * t35621 + t37636;
    t37637
}
