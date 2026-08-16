//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1575;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta383(t1603: f64, t3166: f64, t13939: f64, t381: f64, t1049: f64, t4552: f64, t1052: f64, t1066: f64, t13736: f64, t13743: f64, t14527: f64, t14529: f64, t14532: f64, t3026: f64, t3169: f64, t3207: f64, t388: f64, t4660: f64, t4665: f64, t4694: f64, t225: f64, t4553: f64, t1634: f64, t3206: f64, t3174: f64, t4559: f64, t4555: f64, t4657: f64, t990: f64, t14488: f64, t1060: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14534, t14536, t14538, t14543) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1575(t1603, t3166, t13939, t381, t1049, t4552, t1052, t1066, t13736, t13743, t14527, t14529, t14532, t3026, t3169, t3207, t388, t4660, t4665, t4694);
        let (t14545, t14548, t14549, t14552, t14555, t14562, t14571, t14572) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1576(t225, t4553, t1634, t3206, t3174, t4559, t4555, t4657, t990, t14488, t381, t1060);
    (t14534, t14536, t14538, t14543, t14545, t14548, t14549, t14552, t14555, t14562, t14571, t14572)
}
