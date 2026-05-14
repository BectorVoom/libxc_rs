//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1172/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1172<F: Float>(t34001: F, t34019: F, t34023: F, t34028: F, t34030: F, t34033: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F, t34054: F, t34056: F, t34060: F, t34062: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36789 = 0.13493923611111111112e-4 * t34001;
    let t36793 = 0.94685814672924837674e-4 * t34019;
    let t36794 = 0.41030519691600762993e-3 * t34023;
    let t36795 = 0.89759162297375602412e-9 * t34028;
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
    (t36789, t36793, t36794, t36795, t36796, t36797, t36800, t36801, t36802, t36803, t36804, t36805, t36806, t36807, t36808, t36809)
}
