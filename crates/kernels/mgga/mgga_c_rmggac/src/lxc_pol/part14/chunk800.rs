//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 800/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk800<F: Float>(t289: F, t35124: F, t35128: F, t35130: F, t35132: F, t39491: F, t39493: F, t39495: F, t39497: F, t39499: F, t39506: F, t39507: F, t39514: F, t39518: F, t39523: F, t39525: F, t39529: F, t39531: F) -> (F,) {
    let t39533 = -0.76616279807936110914e-4 * t39491 - 0.25538759935978703638e-4 * t39493 + 0.25538759935978703638e-4 * t39495 + 0.85129199786595678796e-5 * t39497 + 0.1064114997332445985e-4 * t39499 - 0.15243824895787514157e-3 * t35124 + 0.21684485328539747656e-4 * t35128 - 0.90915538847484472429e-2 * t35130 + 0.15965655602485078085e0 * t35132 - t39506 - 0.4726e1 * t289 * t39507 - 0.85129199786595678796e-5 * t39514 - 0.85129199786595678796e-5 * t39518 + 0.53205749866622299248e-5 * t39523 - 0.31923449919973379548e-4 * t39525 - t39529 + 0.17961362552795712846e0 * t39531;
    (t39533,)
}
