//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2959/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2959<F: Float>(t19150: F, t4719: F, t19167: F, t4724: F, t981: F, t19471: F, t18899: F, t23451: F, t41224: F, t23648: F, t4733: F, t23568: F, t3022: F) -> (F, F, F, F, F, F) {
    let t78460 = F::cast_from(0.70178683471615754484e1_f64) * t4719 * t19150;
    let t78463 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t4724 * t19167;
    let t78465 = F::cast_from(0.10389515463408878255e3_f64) * t4719 * t19471;
    let t78469 = F::cast_from(0.12304822629859687989e5_f64) * t981 * t41224 * t23451 * t18899;
    let t78472 = F::cast_from(0.6233709278045326953e3_f64) * t981 * t23648 * t4733;
    let t78474 = F::cast_from(0.51947577317044391277e2_f64) * t3022 * t23568;
    (t78460, t78463, t78465, t78469, t78472, t78474)
}
