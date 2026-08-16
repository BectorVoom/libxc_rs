//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1206/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1206<F: Float>(t39458: F, t39464: F, t39470: F, t37584: F, t37588: F, t38452: F, t39452: F, t39455: F, t39460: F, t39462: F, t39467: F, t39476: F) -> F {
    let t41392 = F::cast_from(0.13869154784086829701e1_f64) * t39458;
    let t41395 = F::cast_from(0.11902492299418487743e0_f64) * t39464;
    let t41397 = F::cast_from(0.28914548798370980346e-3_f64) * t39470;
    let t41401 = F::cast_from(0.34672886960217074252e0_f64) * t39452 - F::cast_from(0.10401866088065122276e1_f64) * t39455 - t41392 + F::cast_from(0.17336443480108537126e0_f64) * t39460 + F::cast_from(0.5200933044032561138e0_f64) * t39462 - t41395 + F::cast_from(0.10401866088065122276e1_f64) * t39467 - t41397 - F::cast_from(0.57131963037208741168e-1_f64) * t37584 - F::cast_from(0.95219938395347901946e-2_f64) * t37588 - t38452 - F::cast_from(0.43663693315433241794e-2_f64) * t39476;
    t41401
}
