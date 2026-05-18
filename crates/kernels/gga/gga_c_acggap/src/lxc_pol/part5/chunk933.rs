//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 933/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk933<F: Float>(t3700: F, t993: F, t1015: F, t173: F, t1029: F, t3670: F, t3645: F, t460: F, t13079: F, t3090: F, t13223: F, t3038: F) -> (F, F, F, F, F, F) {
    let t14421 = F::new(0.34013387707001991332e-1) * t3700 * t993;
    let t14423 = F::new(1.0) / t1015 / t173;
    let t14429 = t3670 * t1029;
    let t14442 = t3645 * t460;
    let t14446 = t13079 * t3090;
    let t14459 = F::new(0.15805078039045227836e2) * t13223 * t3038;
    (t14421, t14423, t14429, t14442, t14446, t14459)
}
