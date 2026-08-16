//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk861;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta192(t182: f64, t5151: f64, t172: f64, t1787: f64, t763: f64, t67: f64, t758: f64, t193: f64, t533: f64, t1845: f64, t3701: f64, t3692: f64, t1307: f64, t1388: f64, t2408: f64, t2417: f64, t2423: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t3918: f64, t5122: f64, t5126: f64, t5127: f64, t5131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5153, t5154, t5155, t5156, t5157, t5158, t5159, t5160, t5161) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk861(t182, t5151, t172, t1787, t763, t67, t758, t193, t533, t1845, t3701);
        let (t5164, t5165) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk862(t3692, t1307, t1388, t2408, t2417, t2423, t3686, t3688, t3690, t3695, t3813, t3918, t5122, t5126, t5127, t5131, t5153, t5156, t5159, t5160, t5161);
    (t5153, t5154, t5155, t5156, t5157, t5158, t5159, t5160, t5161, t5164, t5165)
}
