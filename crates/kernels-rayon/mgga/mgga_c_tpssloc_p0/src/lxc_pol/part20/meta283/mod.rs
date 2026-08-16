//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta283(t2843: f64, t290: f64, t10662: f64, t10702: f64, t10524: f64, t2929: f64, t951: f64, t959: f64, t2904: f64, t2925: f64, t950: f64, t2880: f64, t2888: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10704, t10705, t10707, t10709, t10711, t10713, t10715, t10717) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1477(t2843, t290, t10662, t10702, t10524, t2929, t951, t959, t2904, t2925, t950, t2880, t2888, t931);
    (t10704, t10705, t10707, t10709, t10711, t10713, t10715, t10717)
}
