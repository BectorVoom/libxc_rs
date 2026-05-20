//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3611/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3611<F: Float>(t68415: F, t68429: F, t68443: F, t68461: F, t1132: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68402: F) -> (F, F, F) {
    let t68463 = t68415 + t68429 + t68443 + t68461;
    let t68464 = t1132 * t68463;
    let t68466 = -F::cast_from(0.40256666666666666668e0_f64) * t56187 - F::new(0.12077e1) * t56189 + F::cast_from(0.26837777777777777778e0_f64) * t56209 + F::cast_from(0.13418888888888888889e0_f64) * t56212 + F::cast_from(0.80513333333333333335e0_f64) * t56214 - F::cast_from(0.22364814814814814815e0_f64) * t56216 + F::cast_from(0.53675555555555555558e0_f64) * t56228 - F::cast_from(0.20128333333333333334e0_f64) * t56230 - F::cast_from(0.62621481481481481484e0_f64) * t56236 - F::cast_from(0.20128333333333333334e0_f64) * t68389 + F::new(0.301925e0) * t68393 - F::cast_from(0.40256666666666666666e0_f64) * t68397 + F::cast_from(0.26837777777777777777e0_f64) * t68399 + F::cast_from(0.36793333333333333333e-1_f64) * t68402 + F::new(0.258925e1) * t68464;
    (t68463, t68464, t68466)
}
