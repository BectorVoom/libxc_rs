//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 101/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk101<F: Float>(t25: F, t313: F, t353: F, t344: F, t347: F, t350: F, t346: F, t45: F, t67: F, t222: F, t8: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t355 = t353 * t25 * t313;
    let t357 = F::new(0.379785e1) * t347 + F::new(0.8969e0) * t344 + F::new(0.204775e0) * t350 + F::new(0.24647e0) * t355;
    let t360 = F::new(1.0) + F::new(0.16081824322151104822e2) / t357;
    let t361 = f64::ln(t360);
    let t365 = F::new(1.0) + F::new(0.278125e-1) * t344;
    let t370 = F::new(0.51785e1) * t347 + F::new(0.905775e0) * t344 + F::new(0.1100325e0) * t350 + F::new(0.248355e0) * t355;
    let t373 = F::new(1.0) + F::new(0.29608574643216675549e2) / t370;
    let t374 = f64::ln(t373);
    let t378 = -F::new(0.62182e-1) * t346 * t361 + F::new(0.19751789702565206229e-1) * t45 * t365 * t374;
    let t379 = t67 * t378;
    let t380 = t8 * t222;
    let t381 = pow_1_3::<f64>(t380);
    (t355, t357, t360, t361, t365, t370, t373, t374, t378, t379, t380, t381)
}
