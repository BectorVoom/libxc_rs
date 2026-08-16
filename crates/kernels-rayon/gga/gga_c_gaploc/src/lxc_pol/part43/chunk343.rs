//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 343/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk343(t2792: f64, t492: f64, t105: f64, t1063: f64, t1358: f64, t2268: f64, t2308: f64, t2313: f64, t2319: f64, t2323: f64, t2328: f64, t2738: f64, t2741: f64, t2757: f64, t2762: f64, t2766: f64, t2780: f64, t2784: f64, t2789: f64, t380: f64, t419: f64, t989: f64, t994: f64) -> f64 {
    let t2793 = t492 * t2792;
    let t2796 = 0.37940008847568199465e-1_f64 * t380 * t989 + 0.28455006635676149599e-1_f64 * t419 * t989 - 0.28455006635676149599e-1_f64 * t1063 * t2738 + 0.28455006635676149599e-1_f64 * t2268 * t2741 + 0.28455006635676149599e-1_f64 * t105 * t2757 - 0.31616674039640166221e-2_f64 * t1358 * t2762 - 0.85365019907028448797e-1_f64 * t2268 * t2766 - 0.31616674039640166221e-2_f64 * t2308 + 0.23712505529730124666e-2_f64 * t2313 - 0.23712505529730124666e-2_f64 * t2319 + 0.23712505529730124666e-2_f64 * t2323 - 0.23712505529730124666e-2_f64 * t2328 - 0.37940008847568199465e-1_f64 * t380 * t994 - 0.28455006635676149599e-1_f64 * t419 * t994 + 0.28455006635676149599e-1_f64 * t1063 * t2780 + 0.31616674039640166221e-2_f64 * t1358 * t2784 + 0.56910013271352299198e-1_f64 * t2268 * t2789 - 0.28455006635676149599e-1_f64 * t105 * t2793;
    t2796
}
