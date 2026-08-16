//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1827/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1827(t2250: f64, t3: f64, t1933: f64, t368: f64, t3068: f64, t1058: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t23413 = t3 * t2250;
    let t23414 = t1933 * t23413;
    let t23417 = sigma0 * t368;
    let t23418 = t23417 * t3068;
    let t23419 = t1058 * t23418;
    (t23413, t23414, t23417, t23418, t23419)
}
