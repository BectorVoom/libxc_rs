//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2528;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta700(t1597: f64, t341: f64, t10245: f64, t13847: f64, t2986: f64, t13931: f64, t2987: f64, t135: f64, t13933: f64, t973: f64, t13532: f64, t13784: f64, t10213: f64, t134: f64, t344: f64, t13537: f64, t4509: f64, t4540: f64, t13797: f64, t10186: f64, t13848: f64, t13780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48184, t48189, t48191, t48207, t48210) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2528(t1597, t341, t10245, t13847, t2986, t13931, t2987, t135, t13933, t973, t13532, t13784);
        let (t48213, t48215, t48217, t48221, t48233, t48242) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2529(t10213, t134, t344, t13537, t2986, t4509, t4540, t13797, t1597, t10186, t13848, t13780);
    (t48184, t48189, t48191, t48207, t48210, t48213, t48215, t48217, t48221, t48233, t48242)
}
