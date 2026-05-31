//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3642/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3642<F: Float>(t58207: F, t68454: F, t68529: F, t68532: F, t68535: F, t68538: F, t68540: F, t68543: F, t68546: F, t68548: F, t68550: F, t68553: F, t68556: F, t68559: F, t68561: F) -> F {
    let t68920 = F::cast_from(0.43816888888888888889e0_f64) * t68529 - F::cast_from(0.85199506172839506175e-1_f64) * t68532 + F::cast_from(0.32862666666666666666e0_f64) * t68535 - F::cast_from(0.48685432098765432097e-1_f64) * t58207 - F::cast_from(0.43816888888888888888e0_f64) * t68538 - F::cast_from(0.65725333333333333332e0_f64) * t68540 + F::cast_from(0.16431333333333333333e0_f64) * t68543 + F::cast_from(0.49293999999999999999e0_f64) * t68546 + F::cast_from(0.73028148148148148149e-1_f64) * t68548 + F::cast_from(0.21908444444444444444e0_f64) * t68550 - F::cast_from(0.54771111111111111112e-1_f64) * t68553 + F::cast_from(0.36514074074074074075e-1_f64) * t68556 + F::cast_from(0.5696775e1_f64) * t68559 - F::cast_from(0.3071625e0_f64) * t68561 - F::cast_from(0.79724444444444444445e0_f64) * t68454;
    t68920
}
