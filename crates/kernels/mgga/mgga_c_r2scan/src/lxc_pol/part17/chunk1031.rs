//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1031/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1031<F: Float>(t12960: F, t11380: F, t11454: F, t12437: F, t12440: F, t12443: F, t12944: F, t12947: F, t12949: F, t12953: F, t12958: F, t11357: F, t11566: F, t11570: F, t11580: F, t12721: F, t12723: F, t12726: F, t12728: F, t12730: F, t12733: F, t12939: F, t12942: F) -> (F, F) {
    let t12961 = t12960 / F::new(2.0);
    let t12962 = t11380 + t12944 + t12947 - t12949 + F::cast_from(0.30487649791575028312e-3_f64) * t12437 + t12953 - t11454 - F::cast_from(0.72042316457491791901e-3_f64) * t12440 - F::cast_from(0.1440846329149835838e-2_f64) * t12443 - t12958 + t12961;
    let t12964 = t12721 - F::cast_from(0.60975299583150056624e-3_f64) * t11566 + F::cast_from(0.86737941314158990616e-4_f64) * t11570 - t12723 - t12726 - t12728 - t12730 - t12733 - t11357 + F::cast_from(0.3842256877732895568e-2_f64) * t11580 + t12939 + t12942 + t12962;
    (t12961, t12964)
}
