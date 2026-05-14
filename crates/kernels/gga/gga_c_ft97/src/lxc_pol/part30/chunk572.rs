//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 572/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk572<F: Float>(t2506: F, t27841: F, t1434: F, t193: F, t6837: F, t747: F, t743: F, t6109: F, t1154: F, t6061: F, t1424: F, t3938: F, t27767: F, t9770: F, t446: F, t27753: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27842 = t2506 * t27841;
    let t27844 = t1434 * t193 * t27842;
    let t27845 = t6837 * t747;
    let t27846 = t743 * t27845;
    let t27848 = t6109 * t193 * t27846;
    let t27850 = t6061 * t1154;
    let t27851 = t743 * t27850;
    let t27853 = t6109 * t193 * t27851;
    let t27855 = t1424 * t3938;
    let t27856 = t743 * t27855;
    let t27858 = t6109 * t193 * t27856;
    let t27860 = t9770 * t27767;
    let t27861 = t446 * t27860;
    let t27863 = t9770 * t27753;
    (t27844, t27845, t27848, t27850, t27853, t27858, t27860, t27861, t27863)
}
