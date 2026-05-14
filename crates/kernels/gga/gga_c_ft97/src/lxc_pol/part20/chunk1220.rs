//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1220/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1220<F: Float>(t28735: F, t55543: F, t6318: F, t840: F, t1476: F, t43912: F, t14678: F, t1901: F, t25162: F, t28782: F, t25165: F, t2665: F, t28755: F, t3746: F, t2413: F, t28746: F, t6317: F) -> (F, F, F, F, F, F) {
    let t113099 = t28735 * t840 * t6318 * t55543;
    let t113101 = t43912 * t1476;
    let t113103 = t1901 * t113101 * t14678;
    let t113105 = t25162 * t28782;
    let t113106 = 2.0 * t113105;
    let t113110 = t28755 * t2665 * t25165 * t3746;
    let t113114 = t6317 * t2665 * t28746 * t2413;
    (t113099, t113103, t113105, t113106, t113110, t113114)
}
