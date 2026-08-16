//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta240(t1041: f64, t17659: f64, t4630: f64, t4641: f64, t248: f64, t3101: f64, t5873: f64, t3130: f64, t376: f64, t5866: f64, t2970: f64, t5824: f64, t973: f64, t5828: f64, t10231: f64, t5817: f64, t2989: f64, t5398: f64, t2987: f64, t5836: f64, t5842: f64, t13847: f64, t4514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17660, t17662, t17667, t17668, t17712, t17763) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk895(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5866, t2970, t5824);
        let (t17764, t17770, t17784, t17794, t17800, t17804, t17808) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk896(t17763, t973, t2970, t5828, t10231, t5817, t2989, t5398, t2987, t5836, t5842, t13847, t4514);
    (t17660, t17662, t17667, t17668, t17712, t17764, t17770, t17784, t17794, t17800, t17804, t17808)
}
