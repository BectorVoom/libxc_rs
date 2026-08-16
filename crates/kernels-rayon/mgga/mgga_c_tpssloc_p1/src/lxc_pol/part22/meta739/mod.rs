//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta739 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2434;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2435;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta739(t17513: f64, t49489: f64, t10661: f64, t21253: f64, t912: f64, t2842: f64, t4395: f64, t5695: f64, t10702: f64, t21268: f64, t10817: f64, t21315: f64, t2792: f64, t4396: f64, t5726: f64, t1557: f64, t17422: f64, t10655: f64, t21318: f64, t1556: f64, t60745: f64, t17520: f64, t10771: f64, t14271: f64, t14276: f64, t17535: f64, t17538: f64, t17541: f64, t21259: f64, t2886: f64, t4433: f64, t49430: f64, t5743: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69288, t69291, t69294, t69297, t69299) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2434(t17513, t49489, t10661, t21253, t912, t2842, t4395, t5695, t10702, t21268, t10817, t21315);
        let (t69302, t69305, t69307, t69310, t69313) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2435(t2792, t4396, t5726, t1557, t17422, t10655, t21318, t1556, t2842, t60745, t17520, t4395);
        let t69326 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2436(t10771, t14271, t14276, t17535, t17538, t17541, t21259, t2886, t4433, t49430, t5743, t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69313, t931);
    (t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69313, t69326)
}
