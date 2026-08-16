//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1323;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta238(t761: f64, t9919: f64, t2531: f64, t2535: f64, t32: f64, t717: f64, t1891: f64, t68: f64, t2617: f64, t2629: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64, t2632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9921, t9922, t9929, t9946, t9967, t9970, t9971, t9972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1323(t761, t9919, t2531, t2535, t32, t717, t1891, t68, t2617, t2629, t813, t236);
        let (t9973, t9974, t9975) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1324(t240, t9972, t812, t232, t2632);
    (t9921, t9922, t9929, t9946, t9967, t9970, t9971, t9972, t9973, t9974, t9975)
}
