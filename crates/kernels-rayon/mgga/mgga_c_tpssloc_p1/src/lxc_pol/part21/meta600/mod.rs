//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta600(t2509: f64, t2512: f64, t745: f64, t9711: f64, t1294: f64, t2504: f64, t9493: f64, t2369: f64, t9489: f64, t116: f64, t4: f64, t126: f64, t268: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t39259, t39261, t39263, t39264, t39266, t39267, t39273) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2353(t2509, t2512, t745, t9711, t1294, t2504, t9493, t2369, t9489, t116, t4, t126, t268, t8705);
    (t39259, t39261, t39263, t39264, t39266, t39267, t39273)
}
