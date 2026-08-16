//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1816;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta412(t10231: f64, t4338: f64, t973: f64, t13542: f64, t977: f64, t10388: f64, t10424: f64, t10480: f64, t10876: f64, t10898: f64, t10949: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t13977: f64, t13982: f64, t13987: f64, t13991: f64, t13995: f64, t1618: f64, t3073: f64, t3109: f64, t3130: f64, t4596: f64, t4652: f64, t13546: f64, t13555: f64, t2979: f64, t13528: f64, t13532: f64, t10214: f64, t13537: f64, t13969: f64, t4595: f64, t1616: f64, t2780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13998, t14001, t14004) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1816(t10231, t4338, t973, t13542, t977, t10388, t10424, t10480, t10876, t10898, t10949, t13959, t13963, t13966, t13972, t13977, t13982, t13987, t13991, t13995, t1618, t3073, t3109, t3130, t4596, t4652);
        let (t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1817(t13546, t977, t13555, t2979, t13528, t13532, t10214, t13537, t13969, t4595, t3130, t1616, t2780);
    (t13998, t14001, t14004, t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032)
}
