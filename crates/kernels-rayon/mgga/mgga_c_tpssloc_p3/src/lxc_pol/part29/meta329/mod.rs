//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta329(t11588: f64, t1184: f64, t3451: f64, t3447: f64, t3448: f64, t3475: f64, t1239: f64, t68: f64, t225: f64, t3484: f64, t1222: f64, t3567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11589, t11591, t11593, t11604, t11605, t11606, t11613, t11642) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1387(t11588, t1184, t3451, t3447, t3448, t3475, t1239, t68, t225, t3484, t1222, t3567);
    (t11589, t11591, t11593, t11604, t11605, t11606, t11613, t11642)
}
