//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1240/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1240<F: Float>(t28160: F, t11054: F, t5640: F, t24886: F, t2660: F, t10909: F, t7416: F, t25193: F, t959: F, t7482: F, t8793: F, t32356: F, t723: F) -> (F, F, F, F, F, F, F) {
    let t32936 = F::new(0.15976219147466979032e-1) * t28160;
    let t32937 = t5640 * t11054;
    let t32938 = F::new(0.1533717038156829987e1) * t32937;
    let t32940 = F::new(0.21450293971110256002e1) * t24886 * t2660;
    let t32942 = F::new(0.87421871174939309262e2) * t7416 * t10909;
    let t32943 = t25193 * t959;
    let t32944 = F::new(0.14896037479937677779e-1) * t32943;
    let t32946 = F::new(0.14300195980740170668e1) * t8793 * t7482;
    let t32948 = t32356 * t723;
    (t32936, t32938, t32940, t32942, t32944, t32946, t32948)
}
