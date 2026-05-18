//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1128/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1128<F: Float>(t39506: F, t39509: F, t39512: F, t39514: F, t39517: F, t39520: F, t39523: F, t39524: F, t39526: F, t39529: F, t39532: F, t39535: F) -> F {
    let t39537 = F::new(0.32927245914677557994e0) * t39506 + F::new(0.16463622957338778997e0) * t39509 + t39512 - F::new(0.27439371595564631661e-1) * t39514 + F::new(0.21831846657716620896e-2) * t39517 + F::new(0.26198215989259945076e-1) * t39520 + t39523 + F::new(0.5200933044032561138e0) * t39524 - F::new(0.87327386630866483584e-2) * t39526 - F::new(0.87327386630866483584e-2) * t39529 + F::new(0.13099107994629972538e-1) * t39532 - F::new(0.13099107994629972538e-1) * t39535;
    t39537
}
