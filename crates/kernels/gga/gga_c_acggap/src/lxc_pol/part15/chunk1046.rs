//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1046/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1046<F: Float>(t34091: F, t34095: F, t34099: F, t34101: F, t34107: F, t34130: F, t34158: F, t34170: F, t34172: F, t34175: F, t34189: F, t34204: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36937 = F::new(0.34299214494455789578e-2) * t34091;
    let t36938 = F::new(0.12579236915841660828e-2) * t34095;
    let t36939 = F::new(0.42874018118069736972e-2) * t34099;
    let t36940 = F::new(0.21437009059034868486e-2) * t34101;
    let t36942 = F::new(0.18868855373762491241e-1) * t34107;
    let t36951 = F::new(0.42874018118069736972e-3) * t34130;
    let t36962 = F::new(0.13719685797782315831e-1) * t34158;
    let t36967 = F::new(0.21437009059034868486e-2) * t34170;
    let t36968 = F::new(0.13719685797782315831e-1) * t34172;
    let t36969 = F::new(0.21437009059034868486e-2) * t34175;
    let t36972 = F::new(0.12579236915841660827e-1) * t34189;
    let t36976 = F::new(0.16006300097412701803e-1) * t34204;
    (t36937, t36938, t36939, t36940, t36942, t36951, t36962, t36967, t36968, t36969, t36972, t36976)
}
