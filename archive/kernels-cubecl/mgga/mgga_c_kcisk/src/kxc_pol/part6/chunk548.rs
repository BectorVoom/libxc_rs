//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 548/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk548<F: Float>(t222: F, t227: F, t229: F, t3289: F, t7715: F, t7718: F, t44: F, t7714: F, t291: F, t7710: F, t295: F, t559: F, t294: F, t2071: F, t2351: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t7722 = piecewise3::<F>(t228, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3289 * t7715 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t229 * t7718);
    let t7724 = (t7714 + t7722) * t44;
    let t7725 = t7724 * t291;
    let t7727 = piecewise3::<F>(t223, F::cast_from(0.0_f64), t7710);
    let t7728 = t295 * t7727;
    let t7729 = t7728 * t559;
    let t7730 = t294 * t7729;
    let t7732 = t2071 * t2351;
    (t7724, t7725, t7728, t7730, t7732)
}
