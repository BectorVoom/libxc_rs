//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1248;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta422(t2986: f64, t4514: f64, t61250: f64, t13847: f64, t17794: f64, t17863: f64, t48279: f64, t10231: f64, t21409: f64, t973: f64, t21462: f64, t2970: f64, t10254: f64, t21510: f64, t21472: f64, t13822: f64, t21452: f64, t21468: f64, t42972: f64, t21682: f64, t225: f64, t1009: f64, t21480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69686, t69691, t69699, t69727, t69739) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1248(t2986, t4514, t61250, t13847, t17794, t17863, t48279, t10231, t21409, t973, t21462, t2970);
        let (t69746, t69796, t69801, t69806, t69871, t69923) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1249(t10254, t21510, t21472, t2970, t973, t13822, t21452, t21468, t42972, t21682, t225, t1009, t21480);
    (t69686, t69691, t69699, t69727, t69739, t69746, t69796, t69801, t69806, t69871, t69923)
}
