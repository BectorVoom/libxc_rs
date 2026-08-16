//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1361;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta330(t12110: f64, t2375: f64, t3684: f64, t9882: f64, t9888: f64, t9885: f64, t3824: f64, t588: f64, t1287: f64, t2225: f64, t1284: f64, t2516: f64, t17: f64, t521: f64, t9861: f64, t3826: f64, t592: f64, t1285: f64, t2371: f64, t3691: f64, t1294: f64, t9494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12111, t12114, t12116, t12118, t12120, t12123, t12129) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1361(t12110, t2375, t3684, t9882, t9888, t9885, t3824, t588, t1287, t2225, t1284, t2516);
        let (t12130, t12133, t12134, t12136, t12138, t12141) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1362(t12129, t17, t521, t9861, t3826, t592, t1285, t2225, t2371, t3691, t1294, t9494);
    (t12111, t12114, t12116, t12118, t12120, t12123, t12130, t12133, t12134, t12136, t12138, t12141)
}
