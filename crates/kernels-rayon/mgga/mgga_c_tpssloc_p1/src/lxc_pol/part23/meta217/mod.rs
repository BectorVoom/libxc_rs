//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta217(t3069: f64, t4669: f64, t1612: f64, t3082: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t10277: f64, t3061: f64, t10216: f64, t10969: f64, t10868: f64, t1539: f64, t248: f64, t1041: f64, t1615: f64, t3131: f64, t360: f64, t883: f64, t1573: f64, t2904: f64, t1561: f64, t2885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13995, t14117, t14160, t14164, t14172, t14187) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863(t3069, t4669, t1612, t3082, t1606, t698, t973, t1043, t2770, t10277, t3061, t10216, t10969);
        let (t14202, t14203, t14211, t14219, t14263, t14271) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk864(t10868, t1539, t248, t1041, t1615, t3131, t360, t883, t1573, t2904, t1561, t2885);
    (t13995, t14117, t14160, t14164, t14172, t14187, t14202, t14203, t14211, t14219, t14263, t14271)
}
