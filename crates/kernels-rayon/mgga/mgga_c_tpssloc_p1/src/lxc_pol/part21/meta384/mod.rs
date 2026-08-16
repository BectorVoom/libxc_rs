//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1845;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta384(t13985: f64, t4593: f64, t4582: f64, t3132: f64, t3069: f64, t4669: f64, t10231: f64, t4338: f64, t973: f64, t13542: f64, t977: f64, t10388: f64, t10424: f64, t10480: f64, t10876: f64, t10898: f64, t10949: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t13977: f64, t13982: f64, t1618: f64, t3073: f64, t3109: f64, t3130: f64, t4596: f64, t4652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13986, t13987, t13990, t13991, t13995) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1845(t13985, t4593, t4582, t3132, t3069, t4669);
        let (t13998, t14000, t14004) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1846(t10231, t4338, t973, t13542, t977, t10388, t10424, t10480, t10876, t10898, t10949, t13959, t13963, t13966, t13972, t13977, t13982, t13987, t13991, t13995, t1618, t3073, t3109, t3130, t4596, t4652);
    (t13986, t13987, t13990, t13991, t13995, t13998, t14000, t14004)
}
