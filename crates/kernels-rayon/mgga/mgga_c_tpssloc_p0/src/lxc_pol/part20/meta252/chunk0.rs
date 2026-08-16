//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1379/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1379(t2617: f64, t2629: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64, t2632: f64, t9660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9967 = t2617 * t2629;
    let t9970 = t813 * t813;
    let t9971 = 1.0_f64 / t9970;
    let t9972 = t9971 * t236;
    let t9973 = t9972 * t240;
    let t9974 = t812 * t9973;
    let t9975 = t2632 * t232;
    let t9976 = t9660 * t9975;
    (t9967, t9970, t9971, t9972, t9973, t9974, t9975, t9976)
}
