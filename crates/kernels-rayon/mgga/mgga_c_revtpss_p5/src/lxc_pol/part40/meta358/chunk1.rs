//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1233/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1233(t10673: f64, t10676: f64, t14668: f64, t14675: f64, t14678: f64, t14682: f64, t14690: f64, t14693: f64, t14697: f64, t14703: f64, t14705: f64, t14707: f64, t2745: f64, t4362: f64) -> f64 {
    let t14711 = 0.42874018118069736972e-3_f64 * t4362 * t14668 + t14675 - 0.42874018118069736972e-3_f64 * t2745 * t14678 - 0.21437009059034868486e-3_f64 * t2745 * t14682 - t14690 + 0.17149607247227894789e-2_f64 * t2745 * t14693 + 0.85748036236139473944e-3_f64 * t2745 * t14697 + t14703 + t14705 + 0.17149607247227894789e-2_f64 * t2745 * t14707 + t10673 - 0.14291339372689912324e-3_f64 * t10676;
    t14711
}
