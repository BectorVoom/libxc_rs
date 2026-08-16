//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1302;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta290(t154: f64, t845: f64, t205: f64, t59: f64, t8705: f64, t207: f64, t215: f64, t2570: f64, t782: f64, t2690: f64, t2588: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9558, t9559, t9569, t9572, t9573, t9577, t9579, t9580) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1302(t154, t845, t205, t59, t8705, t207, t215, t2570, t782, t2690, t2588, t21);
    (t9558, t9559, t9569, t9572, t9573, t9577, t9579, t9580)
}
