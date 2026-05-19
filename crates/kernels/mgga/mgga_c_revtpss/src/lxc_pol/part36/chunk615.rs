//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 615/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk615<F: Float>(t6189: F, t973: F, t2994: F, t3001: F, t4571: F, t4620: F, t6094: F, t6098: F, t6102: F, t6114: F, t6121: F, t6127: F, t6129: F, t6133: F, t6136: F, t6139: F) -> (F, F) {
    let t6190 = t6189 * t973;
    let t6205 = -F::new(0.1294625e1) * t6114 + F::new(0.258925e1) * t6121 + t2994 + F::cast_from(0.20128333333333333334e0_f64) * t4571 - F::cast_from(0.20128333333333333333e0_f64) * t6094 + F::new(0.60385e0) * t6098 - F::new(0.301925e0) * t6102 + F::new(0.82524375e-1) * t6127 + F::new(0.16504875e0) * t6129 + t3001 + F::new(0.11038e0) * t4620 - F::new(0.27595e-1) * t6133 + F::new(0.16557e0) * t6136 - F::new(0.82785e-1) * t6139;
    (t6190, t6205)
}
