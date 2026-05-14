//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1176/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1176<F: Float>(t34030: F, t34033: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F, t34054: F, t34056: F, t34060: F, t34062: F, t34036: F, t34066: F, t34069: F, t34071: F) -> (F, F, F, F, F, F) {
    let t36796 = 0.49239311888846044751e-7 * t34030;
    let t36797 = 0.30890995649606120371e-4 * t34033;
    let t36800 = 0.11594181388521408695e-4 * t34038;
    let t36801 = 0.6154913986105755594e-8 * t34043;
    let t36802 = 0.3077456993052877797e-8 * t34046;
    let t36803 = 0.19888696349719110008e-6 * t34048;
    let t36804 = 0.20633616410564056848e-4 * t34050;
    let t36805 = 0.32017370162603252141e-6 * t34052;
    let t36806 = 0.28605695478281987903e-5 * t34054;
    let t36807 = 0.14068374825384584215e-7 * t34056;
    let t36808 = 0.46573198186092908864e-9 * t34060;
    let t36809 = 0.49520679385353736436e-5 * t34062;
    let t36810 = -0.11666621455439814816e-3 * t34036 + t36800 - t36801 - t36802 + t36803 - t36804 + t36805 + t36806 + t36807 + t36808 + t36809;
    let t36812 = 0.67528199161846004232e-6 * t34066;
    let t36813 = 0.40021712703254065176e-7 * t34069;
    let t36814 = 0.40094868252346065012e-6 * t34071;
    (t36796, t36797, t36810, t36812, t36813, t36814)
}
