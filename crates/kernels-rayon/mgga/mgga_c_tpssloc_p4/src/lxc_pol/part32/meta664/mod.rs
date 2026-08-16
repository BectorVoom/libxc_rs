//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2095;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta664(t24574: f64, t27779: f64, t8015: f64, t85660: f64, t27826: f64, t27403: f64, t27389: f64, t8074: f64, t85917: f64, t24826: f64, t27511: f64, t15394: f64, t2127: f64, t221: f64, t11147: f64, t491: f64, t1089: f64, t1751: f64, t7327: f64, t1653: f64, t7330: f64, t85822: f64, t131: f64, t1419: f64, t23598: f64, t467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94700, t94701, t94710, t94759, t94779, t94784, t94787, t94796) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2095(t24574, t27779, t8015, t85660, t27826, t27403, t27389, t8074, t85917, t24826, t27511, t15394, t2127, t221);
        let (t94797, t94837, t94847, t94858) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2096(t11147, t491, t1089, t1751, t7327, t1653, t7330, t85822, t131, t1419, t23598, t467);
    (t94700, t94701, t94710, t94759, t94779, t94784, t94787, t94796, t94797, t94837, t94847, t94858)
}
