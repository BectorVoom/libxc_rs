//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1722;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1723;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta430(t1307: f64, t1377: f64, t1385: f64, t22635: f64, t22633: f64, t154: f64, t835: f64, t3748: f64, t212: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t22637, t22638, t22639, t22641) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1722(t1307, t1377, t1385, t22635, t22633, t154, t835);
        let t22642 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1723(t22641, t3748);
        let t22643 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1724(t212, t562);
    (t22637, t22638, t22639, t22641, t22642, t22643)
}
