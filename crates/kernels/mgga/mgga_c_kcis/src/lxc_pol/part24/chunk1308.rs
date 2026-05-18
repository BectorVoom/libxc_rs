//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1308/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1308<F: Float>(t101077: F, t7703: F, t100841: F, t100843: F, t101072: F, t101136: F, t101141: F, t28932: F, t28952: F, t7693: F, t7711: F, t93592: F, t93737: F, t96508: F, t96534: F) -> F {
    let t101589 = t7703 * t101077;
    let t101606 = -t96508 - F::new(0.46336805555555555557e-3) * t101589 - F::new(0.44218518518518518517e-2) * t100841 - F::new(0.36848765432098765431e-3) * t100843 - F::new(0.46336805555555555556e-3) * t93592 * t101136 + F::new(0.61836467013888888889e-4) * t96534 - F::new(0.92673611111111111112e-3) * t93592 * t101072 - F::new(0.92673611111111111112e-3) * t93592 * t101141 - F::new(0.185671721767578125e-4) * t93737 * t28952 + F::new(0.69505208333333333333e-3) * t28932 * t7711 + F::new(0.69505208333333333333e-3) * t28932 * t7693;
    t101606
}
