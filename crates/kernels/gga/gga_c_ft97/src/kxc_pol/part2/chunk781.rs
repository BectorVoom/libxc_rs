//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 781/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk781<F: Float>(t13827: F, t258: F, t3951: F, t761: F, t766: F, t242: F, t1175: F, t2459: F, t729: F, t1160: F, t737: F, t2609: F, t241: F, t2409: F, t3897: F, t2599: F) -> (F, F, F, F, F, F, F) {
    let t13828 = t13827 * t258;
    let t13830 = t3951 * t761;
    let t13831 = t13830 * t766;
    let t13832 = t242 * t13831;
    let t13836 = t729 * t1175 * t2459;
    let t13839 = t737 * t1160;
    let t13840 = t13839 * t2609;
    let t13844 = t241 * t13827 * t258;
    let t13848 = t3897 * t2409;
    let t13849 = t2599 * t13848;
    (t13828, t13831, t13832, t13836, t13840, t13844, t13849)
}
