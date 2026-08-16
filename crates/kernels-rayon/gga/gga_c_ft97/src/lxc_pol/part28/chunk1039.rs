//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1039/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1039(t136299: f64, t32239: f64, t34468: f64, t2247: f64, t25798: f64, t32151: f64, t136331: f64, t136332: f64, t136468: f64, t136469: f64, t136474: f64, t136475: f64, t136576: f64, t136597: f64, t22513: f64, t25714: f64, t25718: f64, t25760: f64, t25774: f64, t25779: f64, t25799: f64, t25802: f64, t32247: f64, t41: f64, t45499: f64, t48917: f64, t58: f64, t6450: f64, t7205: f64, t93014: f64) -> (f64, f64) {
    let t145123 = t32239 * t136299 * t34468;
    let t145154 = t32151 * t2247 * t25798;
    let t145157 = -0.45497819271775541929e-4_f64 * t145123 + 0.13649345781532662579e-4_f64 * t32239 * t136475 * t25802 - 0.60548059007656442388e-3_f64 * t136331 * t136332 * t25714 - 0.34049924469135802469e-1_f64 * t32247 * t136469 * t25718 + 0.1136661281381420225e-5_f64 * t136597 * t136475 * t25774 + 0.20474018672298993869e-3_f64 * t136474 * t136475 * t25779 + 0.51074886703703703703e-1_f64 * t136468 * t136469 * t25760 + 0.3967677301665257484e-6_f64 * t45499 * t93014 * t41 * t58 * t7205 * t48917 - 0.25537443351851851852e-1_f64 * t136576 * t6450 - 0.51074886703703703704e-1_f64 * t32247 * t25799 + 0.22705522127871165896e-3_f64 * t22513 * t145154;
    (t145154, t145157)
}
