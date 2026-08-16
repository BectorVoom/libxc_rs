//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1379;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta252(t2617: f64, t2629: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64, t2632: f64, t9660: f64, t819: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9967, t9970, t9971, t9972, t9973, t9974, t9975, t9976) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1379(t2617, t2629, t813, t236, t240, t812, t232, t2632, t9660);
        let (t9978, t9981) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1380(t819, t820, t9976, t2632, t9660);
    (t9967, t9970, t9971, t9972, t9973, t9974, t9975, t9976, t9978, t9981)
}
