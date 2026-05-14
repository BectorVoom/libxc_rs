//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 930/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk930<F: Float>(t33996: F, t34013: F, t34027: F, t34031: F, t34035: F, t34037: F, t34052: F, t34081: F, t34091: F, t34095: F, t34099: F, t34101: F, t34107: F, t34130: F, t34158: F, t34170: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36893 = 0.85748036236139473944e-3 * t33996;
    let t36900 = 0.12862205435420921092e-1 * t34013;
    let t36908 = 0.42874018118069736972e-2 * t34027;
    let t36910 = 0.2264262644851498949e-1 * t34031;
    let t36912 = 0.42874018118069736972e-3 * t34035;
    let t36913 = 0.42874018118069736972e-3 * t34037;
    let t36918 = 0.14291339372689912324e-2 * t34052;
    let t36934 = 0.31448092289604152068e-2 * t34081;
    let t36937 = 0.34299214494455789578e-2 * t34091;
    let t36938 = 0.12579236915841660828e-2 * t34095;
    let t36939 = 0.42874018118069736972e-2 * t34099;
    let t36940 = 0.21437009059034868486e-2 * t34101;
    let t36942 = 0.18868855373762491241e-1 * t34107;
    let t36951 = 0.42874018118069736972e-3 * t34130;
    let t36962 = 0.13719685797782315831e-1 * t34158;
    let t36967 = 0.21437009059034868486e-2 * t34170;
    (t36893, t36900, t36908, t36910, t36912, t36913, t36918, t36934, t36937, t36938, t36939, t36940, t36942, t36951, t36962, t36967)
}
