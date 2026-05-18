//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1039/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1039<F: Float>(t136299: F, t32239: F, t34468: F, t2247: F, t25798: F, t32151: F, t136331: F, t136332: F, t136468: F, t136469: F, t136474: F, t136475: F, t136576: F, t136597: F, t22513: F, t25714: F, t25718: F, t25760: F, t25774: F, t25779: F, t25799: F, t25802: F, t32247: F, t41: F, t45499: F, t48917: F, t58: F, t6450: F, t7205: F, t93014: F) -> (F, F) {
    let t145123 = t32239 * t136299 * t34468;
    let t145154 = t32151 * t2247 * t25798;
    let t145157 = -F::new(0.45497819271775541929e-4) * t145123 + F::new(0.13649345781532662579e-4) * t32239 * t136475 * t25802 - F::new(0.60548059007656442388e-3) * t136331 * t136332 * t25714 - F::new(0.34049924469135802469e-1) * t32247 * t136469 * t25718 + F::new(0.1136661281381420225e-5) * t136597 * t136475 * t25774 + F::new(0.20474018672298993869e-3) * t136474 * t136475 * t25779 + F::new(0.51074886703703703703e-1) * t136468 * t136469 * t25760 + F::new(0.3967677301665257484e-6) * t45499 * t93014 * t41 * t58 * t7205 * t48917 - F::new(0.25537443351851851852e-1) * t136576 * t6450 - F::new(0.51074886703703703704e-1) * t32247 * t25799 + F::new(0.22705522127871165896e-3) * t22513 * t145154;
    (t145154, t145157)
}
