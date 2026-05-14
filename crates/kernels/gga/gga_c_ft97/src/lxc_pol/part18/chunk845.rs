//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 845/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk845<F: Float>(t1564: F, t23057: F, t379: F, t5674: F, t1651: F, t5675: F, t1643: F, t7793: F, t1647: F, t1322: F, t1636: F, t89: F, t22862: F, t370: F, t27: F, t375: F, t5700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23059 = t1564 * t23057 * t379;
    let t23060 = t5674 * t23059;
    let t23063 = t1564 * t5675 * t1651;
    let t23064 = t5674 * t23063;
    let t23067 = t7793 * t5675 * t1643;
    let t23068 = t5674 * t23067;
    let t23071 = t1564 * t5675 * t1647;
    let t23072 = t5674 * t23071;
    let t23075 = t89 * t1636 * t1322;
    let t23076 = 4.0 / 9.0 * t23075;
    let t23077 = t370 * t22862;
    let t23079 = t89 * t27 * t23077;
    let t23081 = t89 * t375 * t5700;
    (t23059, t23060, t23063, t23064, t23067, t23068, t23071, t23072, t23075, t23076, t23077, t23079, t23081)
}
