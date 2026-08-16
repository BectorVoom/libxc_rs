//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1456;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta295(t13784: f64, t4338: f64, t2986: f64, t10190: f64, t4514: f64, t10213: f64, t60: f64, t344: f64, t135: f64, t340: f64, t4548: f64, t973: f64, t2970: f64, t4522: f64, t10254: f64, t3961: f64, t10236: f64, t10189: f64, t1597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13787, t13790, t13797, t13798, t13822) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1456(t13784, t4338, t2986, t10190, t4514, t10213, t60, t344, t135, t340);
        let (t13825, t13830, t13835, t13839, t13847) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1457(t13822, t4548, t973, t2970, t4522, t10254, t3961, t10236, t10189, t1597);
    (t13787, t13790, t13797, t13798, t13822, t13825, t13830, t13835, t13839, t13847)
}
