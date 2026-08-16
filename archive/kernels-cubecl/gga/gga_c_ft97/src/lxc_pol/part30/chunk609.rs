//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 609/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk609<F: Float>(t1424: F, t3821: F, t2506: F, t1434: F, t193: F, t6837: F, t747: F, t743: F, t6109: F, t1154: F, t6061: F, t3938: F) -> (F, F, F, F, F, F, F) {
    let t27841 = t1424 * t3821;
    let t27842 = t2506 * t27841;
    let t27844 = t1434 * t193 * t27842;
    let t27845 = t6837 * t747;
    let t27846 = t743 * t27845;
    let t27848 = t6109 * t193 * t27846;
    let t27850 = t6061 * t1154;
    let t27851 = t743 * t27850;
    let t27853 = t6109 * t193 * t27851;
    let t27855 = t1424 * t3938;
    (t27841, t27844, t27845, t27848, t27850, t27853, t27855)
}
