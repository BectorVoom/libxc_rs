//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 840/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk840<F: Float>(t2617: F, t2629: F, t813: F, t236: F, t240: F, t812: F, t232: F, t2632: F, t9660: F, t819: F, t820: F, t2639: F, t2686: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9967 = t2617 * t2629;
    let t9970 = t813 * t813;
    let t9971 = F::cast_from(1.0_f64) / t9970;
    let t9972 = t9971 * t236;
    let t9973 = t9972 * t240;
    let t9974 = t812 * t9973;
    let t9975 = t2632 * t232;
    let t9976 = t9660 * t9975;
    let t9978 = t819 * t820 * t9976;
    let t9981 = t9660 * t2632;
    let t9983 = t819 * t820 * t9981;
    let t9986 = t2639 * t2686;
    (t9967, t9970, t9971, t9972, t9973, t9974, t9975, t9976, t9978, t9981, t9983, t9986)
}
