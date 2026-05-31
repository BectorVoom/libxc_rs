//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3818/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3818<F: Float>(t48255: F, t46999: F, t47005: F, t47007: F, t1448: F, t5591: F, t48260: F, t48262: F, t13648: F, t13716: F, t22496: F, t39773: F, t39783: F, t4139: F, t47003: F, t5532: F, t5542: F) -> (F, F, F, F, F, F, F) {
    let t73384 = F::cast_from(0.23392894490538584828e1_f64) * t48255;
    let t73388 = F::cast_from(192.0_f64) * t46999;
    let t73389 = F::cast_from(48.0_f64) * t47005;
    let t73390 = F::cast_from(96.0_f64) * t47007;
    let t73394 = t5591 * t1448;
    let t73398 = F::cast_from(0.46785788981077169656e1_f64) * t48260;
    let t73399 = F::cast_from(0.11696447245269292414e1_f64) * t48262;
    let t73400 = -F::cast_from(12.0_f64) * t13648 * t22496 * t4139 + F::cast_from(6.0_f64) * t13716 * t4139 * t5532 - F::cast_from(12.0_f64) * t4139 * t5542 * t73394 + t39773 - t39783 + t47003 - t73384 + t73388 + t73389 + t73390 + t73398 - t73399;
    (t73384, t73388, t73389, t73390, t73398, t73399, t73400)
}
