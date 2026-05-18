//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1175/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1175<F: Float>(t39885: F, t8243: F, t2605: F, t37699: F, t10833: F, t980: F, t38069: F, t38074: F, t38076: F, t38079: F, t40090: F, t40092: F, t40095: F, t40098: F, t40100: F) -> F {
    let t40102 = t39885 * t8243;
    let t40103 = F::new(0.19514881078765566037e-1) * t40102;
    let t40107 = t37699 * t2605;
    let t40109 = t980 * t10833;
    let t40111 = F::new(0.55889527443754549494e0) * t40090 + F::new(0.10401866088065122276e1) * t40092 + F::new(0.13002332610081402845e0) * t40095 + F::new(0.43663693315433241792e-2) * t40098 - F::new(0.13099107994629972538e-1) * t40100 + t40103 - t38069 + F::new(0.34672886960217074253e0) * t38074 + F::new(0.69345773920434148506e0) * t38076 + F::new(0.11557628986739024751e0) * t38079 + F::new(0.29272321618148349056e-1) * t40107 + F::new(0.42377972951376424087e0) * t40109;
    t40111
}
