//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 94/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk94<F: Float>(t303: F, t306: F, t309: F, t315: F, t325: F, t323: F, t45: F) -> (F, F, F, F, F) {
    let t330 = F::new(0.51785e1) * t306 + F::new(0.905775e0) * t303 + F::new(0.1100325e0) * t309 + F::new(0.1241775e0) * t315;
    let t333 = F::new(1.0) + F::new(0.29608574643216675549e2) / t330;
    let t334 = f64::ln(t333);
    let t335 = t325 * t334;
    let t338 = -t323 + F::new(0.19751789702565206229e-1) * t45 * t335;
    (t330, t333, t334, t335, t338)
}
