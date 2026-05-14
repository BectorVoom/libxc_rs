//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 942/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk942<F: Float>(t36175: F, t36177: F, t36194: F, t36198: F, t36205: F, t36207: F, t36210: F, t36214: F, t36240: F, t36273: F, t36283: F, t36286: F, t36292: F, t36299: F, t36302: F, t36331: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37898 = 0.18868855373762491241e-2 * t36175;
    let t37899 = 0.68598428988911579156e-2 * t36177;
    let t37904 = 0.62896184579208304136e-2 * t36194;
    let t37905 = 0.94344276868812456204e-2 * t36198;
    let t37907 = 0.36675e0 * t36205;
    let t37908 = 0.183375e0 * t36207;
    let t37909 = 0.183375e0 * t36210;
    let t37910 = 0.183375e0 * t36214;
    let t37924 = 0.16006300097412701803e-1 * t36240;
    let t37934 = 0.21437009059034868486e-2 * t36273;
    let t37937 = 0.85748036236139473944e-3 * t36283;
    let t37938 = 0.34299214494455789578e-1 * t36286;
    let t37941 = 0.21437009059034868486e-2 * t36292;
    let t37944 = 0.28582678745379824648e-2 * t36299;
    let t37945 = 0.17149607247227894789e-2 * t36302;
    let t37960 = 0.17149607247227894789e-2 * t36331;
    (t37898, t37899, t37904, t37905, t37907, t37908, t37909, t37910, t37924, t37934, t37937, t37938, t37941, t37944, t37945, t37960)
}
