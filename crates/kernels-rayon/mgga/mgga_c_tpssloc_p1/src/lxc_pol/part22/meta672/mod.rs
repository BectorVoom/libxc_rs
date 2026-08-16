//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2227;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta672(t17152: f64, t2986: f64, t48213: f64, t17863: f64, t42837: f64, t10186: f64, t17808: f64, t10236: f64, t17635: f64, t13835: f64, t13847: f64, t13839: f64, t48279: f64, t17748: f64, t17849: f64, t2960: f64, t5838: f64, t698: f64, t973: f64, t5844: f64, t4509: f64, t5836: f64, t10190: f64, t17794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61261, t61264, t61273, t61279, t61288, t61291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2227(t17152, t2986, t48213, t17863, t42837, t10186, t17808, t10236, t17635, t13835, t13847, t13839, t48279);
        let (t61294, t61307, t61310, t61313, t61322, t61327) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2228(t13847, t17748, t2986, t17849, t2960, t5838, t698, t973, t5844, t4509, t5836, t10190, t17794);
    (t61261, t61264, t61273, t61279, t61288, t61291, t61294, t61307, t61310, t61313, t61322, t61327)
}
