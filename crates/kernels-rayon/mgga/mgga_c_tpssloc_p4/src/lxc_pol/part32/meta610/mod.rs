//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta610(t2690: f64, t6612: f64, t812: f64, t831: f64, t59: f64, t9971: f64, t23040: f64, t2617: f64, t23061: f64, t6604: f64, t1891: f64, t1895: f64, t213: f64, t39041: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t81807, t81808, t81816, t81824, t81835, t81849) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2009(t2690, t6612, t812, t831, t59, t9971, t23040, t2617, t23061, t6604, t1891, t1895, t213, t39041);
    (t81807, t81808, t81816, t81824, t81835, t81849)
}
