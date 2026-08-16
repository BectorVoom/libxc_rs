//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 935/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk935<F: Float>(t30045: F, t30047: F, t30050: F, t30055: F, t30077: F, t30080: F, t30083: F, t30088: F, t30170: F, t30180: F, t30183: F, t30191: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32339 = F::cast_from(0.94344276868812456207e-3_f64) * t30045;
    let t32340 = F::cast_from(0.94344276868812456207e-3_f64) * t30047;
    let t32341 = F::cast_from(0.12579236915841660828e-2_f64) * t30050;
    let t32342 = F::cast_from(0.31448092289604152069e-3_f64) * t30055;
    let t32348 = F::cast_from(0.15117061203111996147e0_f64) * t30077;
    let t32349 = F::cast_from(0.12004725073059526352e-1_f64) * t30080;
    let t32350 = F::cast_from(0.85748036236139473944e-3_f64) * t30083;
    let t32352 = F::cast_from(0.68026775414003982662e-1_f64) * t30088;
    let t32377 = F::cast_from(0.39624596284901231606e-1_f64) * t30170;
    let t32379 = F::cast_from(0.32100349018573719666e-1_f64) * t30180;
    let t32380 = F::cast_from(0.51448821741683684367e-2_f64) * t30183;
    let t32384 = F::cast_from(0.27010631414383934293e-1_f64) * t30191;
    (t32339, t32340, t32341, t32342, t32348, t32349, t32350, t32352, t32377, t32379, t32380, t32384)
}
