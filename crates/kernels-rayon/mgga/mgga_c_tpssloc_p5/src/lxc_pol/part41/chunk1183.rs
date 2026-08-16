//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1183/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1183(t1102: f64, t18761: f64, t11137: f64, t14818: f64, t18227: f64, t18239: f64, t18497: f64, t18500: f64, t18503: f64, t18508: f64, t18510: f64, t18515: f64, t18518: f64) -> (f64, f64) {
    let t18762 = t18761 * t1102;
    let t18783 = 0.12077e1_f64 * t18227 + 0.36793333333333333333e-1_f64 * t14818 - 0.27595e-1_f64 * t18515 + 0.36793333333333333333e-1_f64 * t18497 + 0.16557e0_f64 * t18518 + 0.13418888888888888889e0_f64 * t11137 + 0.60385e0_f64 * t18239 - 0.5519e-1_f64 * t18503 - 0.16557e0_f64 * t18500 + 0.33114e0_f64 * t18510 + 0.49671e0_f64 * t18508;
    (t18762, t18783)
}
