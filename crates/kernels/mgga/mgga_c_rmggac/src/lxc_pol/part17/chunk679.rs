//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 679/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk679<F: Float>(t9810: F, t1763: F, t7577: F, t739: F, t2289: F, t2412: F, t1942: F, t1986: F, t675: F, t2310: F, t1923: F, t2131: F) -> (F, F, F, F, F, F, F, F) {
    let t9811 = F::cast_from(0.68186654135613354322e-2_f64) * t9810;
    let t9812 = t7577 * t1763;
    let t9813 = t739 * t9812;
    let t9814 = F::cast_from(0.2993560425465952141e-1_f64) * t9813;
    let t9815 = t2412 * t2289;
    let t9816 = F::cast_from(0.25538759935978703638e-4_f64) * t9815;
    let t9817 = t1986 * t1942;
    let t9818 = t675 * t9817;
    let t9819 = F::cast_from(0.12769379967989351819e-4_f64) * t9818;
    let t9820 = t2412 * t2310;
    let t9821 = F::cast_from(0.85129199786595678796e-5_f64) * t9820;
    let t9822 = t1923 * t2131;
    (t9811, t9812, t9814, t9816, t9817, t9819, t9821, t9822)
}
