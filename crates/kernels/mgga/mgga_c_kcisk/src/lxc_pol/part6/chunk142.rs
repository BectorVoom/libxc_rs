//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 142/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk142<F: Float>(t303: F, t306: F, t309: F, t315: F, t240: F, t323: F, t335: F, t507: F) -> (F, F, F, F) {
    let t512 = F::new(0.705945e1) * t306 + F::new(0.1549425e1) * t303 + F::new(0.420775e0) * t309 + F::new(0.1562925e0) * t315;
    let t515 = F::new(1.0) + F::new(0.32164683177870697974e2) / t512;
    let t516 = f64::ln(t515);
    let t524 = -t323 + t240 * (-F::new(0.3109e-1) * t507 * t516 + t323 - F::new(0.19751789702565206229e-1) * t335) + F::new(0.19751789702565206229e-1) * t240 * t335;
    (t512, t515, t516, t524)
}
