//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3877/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3877(t22081: f64, t9962: f64, t22276: f64, t3989: f64, t22281: f64, t22056: f64, t9765: f64, t48865: f64, t48868: f64, t48872: f64, t48876: f64, t48879: f64, t48881: f64, t48888: f64) -> f64 {
    let t74498 = t9962 * t22081;
    let t74505 = t3989 * t22276;
    let t74507 = t3989 * t22281;
    let t74511 = t9765 * t22056;
    let t74513 = -0.80031500487063509015e-2_f64 * t74498 - 0.11433071498151929859e-3_f64 * t48865 + 0.10164000561857065645e-4_f64 * t48868 + 0.36143185997963725434e-3_f64 * t48872 - 0.20328001123714131289e-4_f64 * t48876 + 0.16264433699083676445e-3_f64 * t48879 + 0.24009450146119052704e0_f64 * t74505 - 0.80031500487063509015e-1_f64 * t74507 + 0.60976381323476959249e-3_f64 * t48881 + 0.24009450146119052705e0_f64 * t48888 + 0.54208002996571016773e-3_f64 * t74511;
    t74513
}
