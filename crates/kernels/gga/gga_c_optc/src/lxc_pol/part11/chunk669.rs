//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 669/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk669<F: Float>(t6319: F, t88: F, t2041: F, t538: F, t6163: F, t36: F, t1872: F, t539: F, t2229: F, t740: F, t1820: F, t1828: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6320 = t6319 * t88;
    let t6321 = F::new(144.0) * t6320;
    let t6322 = t538 * t2041;
    let t6323 = t6322 * t88;
    let t6324 = F::new(240.0) * t6323;
    let t6325 = F::new(1.0) / t6163;
    let t6326 = t36 * t6325;
    let t6328 = F::new(120.0) * t6326 * t88;
    let t6329 = t539 * t1872;
    let t6330 = F::new(12.0) * t6329;
    let t6332 = F::new(7.0) / F::new(2.0) * t2229 * t740;
    let t6343 = t1820 * t1828;
    (t6321, t6322, t6324, t6325, t6326, t6328, t6330, t6332, t6343)
}
