//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1938;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta619(t22779: f64, t26292: f64, t1339: f64, t54258: f64, t550: f64, t6936: f64, t22827: f64, t3788: f64, t3792: f64, t54068: f64, t12289: f64, t3791: f64, t54014: f64, t16311: f64, t1825: f64, t26288: f64, t3734: f64, t16314: f64, t26309: f64, t16227: f64, t22833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91225, t91229, t91233, t91237) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1938(t22779, t26292, t1339, t54258, t550, t6936, t22827, t3788, t3792, t54068, t12289, t3791, t54014);
        let (t91241, t91256, t91261, t91263) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1939(t16311, t3788, t3791, t6936, t1339, t1825, t26288, t3734, t16314, t26309, t16227, t22833);
    (t91225, t91229, t91233, t91237, t91241, t91256, t91261, t91263)
}
