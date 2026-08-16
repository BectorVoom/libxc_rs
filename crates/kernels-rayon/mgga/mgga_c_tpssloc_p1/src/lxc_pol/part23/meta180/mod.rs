//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta180(t761: f64, t9919: f64, t1891: f64, t68: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64, t2632: f64, t597: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9921, t9946, t9970, t9971, t9972, t9973, t9974, t9975, t10021) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk808(t761, t9919, t1891, t68, t813, t236, t240, t812, t232, t2632, t597, t61);
    (t9921, t9946, t9970, t9971, t9972, t9973, t9974, t9975, t10021)
}
