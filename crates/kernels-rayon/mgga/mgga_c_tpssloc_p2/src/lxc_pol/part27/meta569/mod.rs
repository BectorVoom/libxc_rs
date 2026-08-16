//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2013;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta569(t22674: f64, t22686: f64, t80681: f64, t22663: f64, t6883: f64, t225: f64, t22624: f64, t22622: f64, t214: f64, t3879: f64, t22675: f64, t22724: f64, t22716: f64, t6903: f64, t22662: f64, t6897: f64, t22684: f64, t6546: f64, t22687: f64, t131: f64, t1365: f64, t22648: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80683, t80689, t80699, t80704, t80707, t80711) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2013(t22674, t22686, t80681, t22663, t6883, t225, t22624, t22622, t214, t3879, t22675, t22724);
        let (t80722, t80725, t80727, t80728, t80730, t80738) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2014(t22716, t6903, t22662, t22674, t6897, t22684, t6546, t22687, t131, t1365, t22648, t794);
    (t80683, t80689, t80699, t80704, t80707, t80711, t80722, t80725, t80727, t80728, t80730, t80738)
}
