//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2115/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2115<F: Float>(t18495: F, t7045: F, t18500: F, t18618: F, t7038: F, t18466: F, t25270: F, t103302: F, t103305: F, t92996: F, t92998: F, t93000: F, t93001: F, t93008: F, t93013: F, t93016: F) -> F {
    let t106068 = t7045 * t18495;
    let t106070 = t7045 * t18500;
    let t106072 = t7038 * t18618;
    let t106074 = t25270 * t18466;
    let t106078 = -F::cast_from(0.51448821741683684367e-1_f64) * t106068 + F::cast_from(0.17149607247227894789e-1_f64) * t106070 - F::cast_from(0.42874018118069736972e-3_f64) * t106072 + t103302 - t92996 - t103305 - F::cast_from(0.42874018118069736972e-3_f64) * t106074 - t92998 + t93000 - F::cast_from(0.60976381323476959248e-3_f64) * t93001 + t93008 - t93013 - F::cast_from(0.90357964994909313586e-5_f64) * t93016;
    t106078
}
