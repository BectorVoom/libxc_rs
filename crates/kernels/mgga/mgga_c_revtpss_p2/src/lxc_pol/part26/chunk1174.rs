//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1174/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1174<F: Float>(t92988: F, t92995: F, t92997: F, t92999: F, t93007: F, t93012: F, t92979: F, t92982: F, t92984: F, t92991: F, t93001: F, t93004: F, t93010: F, t93016: F) -> F {
    let t95671 = F::cast_from(0.3252886739816735289e-3_f64) * t92988;
    let t95673 = F::new(455.0) / F::new(648.0) * t92995;
    let t95674 = F::cast_from(0.15117061203111996147e0_f64) * t92997;
    let t95675 = F::cast_from(0.51384669507166276316e-2_f64) * t92999;
    let t95678 = F::cast_from(0.80328230880474379779e-6_f64) * t93007;
    let t95680 = F::cast_from(0.45178982497454656792e-6_f64) * t93012;
    let t95682 = -F::new(7.0) / F::new(8.0) * t92979 - t92982 / F::new(2.0) + F::new(3.0) / F::new(8.0) * t92984 - t95671 + F::cast_from(0.12196800674228478774e-3_f64) * t92991 - t95673 - t95674 + t95675 - F::cast_from(0.3658582879408617555e-2_f64) * t93001 + F::cast_from(0.34299214494455789577e-3_f64) * t93004 + t95678 - F::cast_from(0.17149607247227894789e-2_f64) * t93010 - t95680 - F::cast_from(0.54214778996945588151e-4_f64) * t93016;
    t95682
}
