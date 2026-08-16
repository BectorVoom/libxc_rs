//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1760/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1760<F: Float>(t3434: F, t421: F, t12228: F, t12227: F, t1187: F, t3495: F, t3516: F, t1196: F, t1130: F, t3376: F, t1151: F, t3379: F, t3428: F) -> (F, F, F, F, F, F, F, F) {
    let t12230 = F::cast_from(1.0_f64) / t3434 / t421;
    let t12231 = t12228 * t12230;
    let t12233 = F::cast_from(0.51726012919273400301e3_f64) * t12227 * t12231;
    let t12234 = t3495 * t1187;
    let t12235 = t12234 * t3516;
    let t12237 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t12235;
    let t12238 = t3376 * t1130;
    let t12240 = F::cast_from(3.0_f64) * t12238 * t1151;
    let t12242 = F::cast_from(3.0_f64) * t3379 * t3428;
    (t12230, t12231, t12233, t12235, t12237, t12238, t12240, t12242)
}
