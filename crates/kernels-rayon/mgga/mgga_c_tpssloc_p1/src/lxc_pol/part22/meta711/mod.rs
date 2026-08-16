//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2308;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta711(t58021: f64, t46278: f64, t1484: f64, t4303: f64, t16634: f64, t4205: f64, t40738: f64, t40754: f64, t12895: f64, t2522: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t4307: f64, t5544: f64, t40761: f64, t16689: f64, t4101: f64, t16701: f64, t20741: f64, t706: f64, t708: f64, t20234: f64, t751: f64, t9897: f64, t13133: f64, t5597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67162, t67163, t67169, t67170, t67174, t67175) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2308(t58021, t46278, t1484, t4303, t16634, t4205, t40738, t40754, t12895, t2522, t40741, t40743, t40748, t40760, t4307, t5544);
        let (t67176, t67178, t67180, t67183, t67186, t67191) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2309(t40761, t16689, t4101, t16701, t4205, t20741, t706, t708, t20234, t751, t9897, t13133, t5597);
    (t67162, t67163, t67169, t67170, t67174, t67175, t67176, t67178, t67180, t67183, t67186, t67191)
}
