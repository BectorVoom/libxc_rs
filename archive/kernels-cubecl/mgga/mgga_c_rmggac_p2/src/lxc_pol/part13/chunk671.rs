//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 671/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk671<F: Float>(t265: F, t570: F, t2079: F, t262: F, t2068: F, t8705: F, t2073: F, t8701: F, t1652: F, t36: F, t118: F, t4616: F) -> (F, F, F, F, F, F, F) {
    let t8915 = t265 * t570;
    let t8917 = t2079 * t262 * t8915;
    let t8919 = t2068 * t8705;
    let t8921 = t2073 * t8701;
    let t8924 = t36 * t1652;
    let t8926 = t2079 * t262 * t8924;
    let t8940 = t118 * t4616;
    (t8915, t8917, t8919, t8921, t8924, t8926, t8940)
}
