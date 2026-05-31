//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1206/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1206<F: Float>(t32339: F, t32340: F, t32341: F, t32342: F, t33874: F, t33876: F, t36836: F, t38781: F, t38787: F, t38792: F, t38796: F, t38799: F, t38801: F, t38805: F, t38810: F, t38815: F, t38817: F) -> F {
    let t41360 = -F::cast_from(0.42874018118069736972e-3_f64) * t38781 + F::cast_from(0.31448092289604152069e-3_f64) * t38787 - F::cast_from(0.21437009059034868486e-2_f64) * t38792 + t32339 + t32340 + t32341 + t32342 - F::cast_from(0.21437009059034868486e-2_f64) * t33874 - F::cast_from(0.36014175219178579057e-1_f64) * t33876 + F::cast_from(0.916875e-1_f64) * t38796 + F::cast_from(0.61125e-1_f64) * t38799 - F::cast_from(0.62896184579208304138e-3_f64) * t38801 - F::cast_from(0.62896184579208304138e-3_f64) * t38805 - F::cast_from(0.62896184579208304138e-3_f64) * t38810 - F::cast_from(0.41930789719472202759e-3_f64) * t38815 - t36836 - F::cast_from(0.85748036236139473944e-3_f64) * t38817;
    t41360
}
