//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1025/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1025<F: Float>(t8563: F, t8565: F, t8569: F, t9353: F, t8572: F, t8574: F, t8109: F, t8110: F, t8111: F, t8112: F, t8113: F, t8114: F, t8117: F, t8118: F) -> (F, F, F, F) {
    let t42450 = F::new(0.5454932330849068346e-1) * t8563;
    let t42451 = F::new(0.13637330827122670865e-1) * t8565;
    let t42452 = F::new(0.13637330827122670865e-1) * t8569;
    let t42454 = F::new(0.11974241701863808564e0) * t9353;
    let t42455 = F::new(0.1702583995731913576e-4) * t8572;
    let t42456 = F::new(0.5107751987195740728e-4) * t8574;
    let t42457 = t42454 + t8109 + t8110 - t8111 - t8112 - t8113 - t8114 - t42455 + t42456 - t8117 + t8118;
    (t42450, t42451, t42452, t42457)
}
