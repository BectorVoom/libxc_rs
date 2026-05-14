//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 591/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk591<F: Float>(t2073: F, t8713: F, t265: F, t570: F, t2079: F, t262: F, t2068: F, t8705: F, t8701: F, t1652: F, t36: F, t118: F, t4616: F, t305: F, t8821: F, t797: F, t8884: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8913 = t2073 * t8713;
    let t8915 = t265 * t570;
    let t8917 = t2079 * t262 * t8915;
    let t8919 = t2068 * t8705;
    let t8921 = t2073 * t8701;
    let t8924 = t36 * t1652;
    let t8926 = t2079 * t262 * t8924;
    let t8940 = t118 * t4616;
    let t8944 = t305 * t8821;
    let t8966 = t797 * t8884;
    (t8913, t8915, t8917, t8919, t8921, t8926, t8940, t8944, t8966)
}
