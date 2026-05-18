//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1058/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1058<F: Float>(t35850: F, t35909: F, t35911: F, t35913: F, t35915: F, t35917: F, t35919: F, t35926: F, t35930: F, t35934: F, t35951: F, t35961: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37744 = F::new(0.1120625e0) * t35850;
    let t37777 = F::new(0.916875e-1) * t35909;
    let t37778 = F::new(0.916875e-1) * t35911;
    let t37779 = F::new(0.61125e-1) * t35913;
    let t37780 = F::new(0.61125e-1) * t35915;
    let t37781 = F::new(0.34299214494455789578e-2) * t35917;
    let t37782 = F::new(0.34299214494455789578e-2) * t35919;
    let t37787 = F::new(0.64025200389650807212e-1) * t35926;
    let t37789 = F::new(0.85748036236139473944e-3) * t35930;
    let t37790 = F::new(0.42874018118069736972e-3) * t35934;
    let t37801 = F::new(0.34299214494455789578e-2) * t35951;
    let t37807 = F::new(0.34299214494455789578e-2) * t35961;
    (t37744, t37777, t37778, t37779, t37780, t37781, t37782, t37787, t37789, t37790, t37801, t37807)
}
