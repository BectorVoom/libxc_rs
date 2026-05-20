//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1472/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1472<F: Float>(t11409: F, t11461: F, t11525: F, t2962: F, t2971: F, t3012: F, t3014: F, t41445: F, t41464: F, t41570: F, t41573: F, t41577: F, t41580: F, t41582: F, t41585: F, t41591: F, t41657: F, t41832: F, t41841: F, t41845: F, t41847: F, t41849: F, t965: F, t972: F, t973: F) -> F {
    let t41853 = -F::cast_from(0.11579025239058625248e4_f64) * t11409 * t2971 * t2962 + t41570 + F::cast_from(0.2077903092681775651e3_f64) * t11461 * t11525 + F::cast_from(0.69263436422725855036e2_f64) * t3012 * t41832 * t972 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t41445 * t973 - t41573 - t41577 - t41580 - t41582 - t41585 + t41591 - t41657 - t41841 - t41845 + t41847 - t41849 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t41464 * t3014;
    t41853
}
