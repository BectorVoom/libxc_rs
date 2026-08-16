//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 100/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk100<F: Float>(t155: F, t153: F, t135: F, t145: F, t455: F, t458: F, t456: F, t459: F, t134: F) -> (F, F, F, F, F, F) {
    let t462 = t155 * t155;
    let t463 = F::cast_from(1.0_f64) / t462;
    let t464 = t153 * t463;
    let t465 = t464 * t135;
    let t467 = t455 * t145 * t458;
    let t470 = -F::cast_from(7.0_f64) / F::cast_from(128.0_f64) * t456 * t145 * t459 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t465 * t467;
    let t471 = F::ln(t134);
    (t462, t463, t464, t467, t470, t471)
}
