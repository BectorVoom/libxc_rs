//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1188/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1188<F: Float>(t213: F, t26473: F, t10982: F, t2061: F, t9646: F, t25365: F, t26544: F, t93190: F, t95726: F, t10638: F, t10978: F, t231: F, t25383: F, t26441: F, t2645: F, t26547: F, t2772: F, t7070: F, t7076: F, t7398: F, t7403: F, t887: F, t95866: F, t95872: F, t95876: F, t95888: F, t95891: F, t95893: F) -> F {
    let t95894 = t213 * t26473;
    let t95899 = F::cast_from(0.19637199382202157274e-3_f64) * t9646 * t2061 * t10982;
    let t95900 = t25365 * t26544;
    let t95902 = t93190 * t95726;
    let t95904 = F::cast_from(0.29272321618148349057e-1_f64) * t95866 + F::cast_from(0.39512695097613069591e1_f64) * t26547 * t2772 + F::cast_from(0.26020884564615598386e1_f64) * t25383 * t26441 + F::cast_from(0.43368140941025997312e-1_f64) * t95872 - F::cast_from(0.65854491829355115987e0_f64) * t7403 * t10978 + F::cast_from(0.21684070470512998656e-1_f64) * t95876 + F::cast_from(0.13010442282307799193e1_f64) * t7070 * t7076 * t7398 * t2645 * t231 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t2061 * t10638 * t231 + F::cast_from(0.51405703062096148812e-1_f64) * t95888 + t95891 - t95893 - F::cast_from(0.19756347548806534796e1_f64) * t95894 * t887 + t95899 - F::cast_from(0.77108554593144223218e-1_f64) * t95900 + F::cast_from(0.13709901006661042888e-1_f64) * t95902;
    t95904
}
