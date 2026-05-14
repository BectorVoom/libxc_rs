//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 804/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk804<F: Float>(t3889: F, t868: F, t441: F, t848: F, t464: F, t3037: F, t3922: F, t449: F, t463: F, t1220: F, t1221: F, t863: F, t864: F, t879: F, t317: F, t3915: F, t3937: F) -> (F, F, F, F, F, F, F, F) {
    let t12218 = t868 * t3889;
    let t12224 = t848 * t441;
    let t12225 = t12224 * t464;
    let t12229 = t3922 * t449 * t3037 * t463;
    let t12233 = t863 * t1220 * t864 * t1221;
    let t12235 = t864 * t879;
    let t12238 = 0.23707617058567841754e2 * t3922 * t317 * t12235;
    let t12240 = 0.15805078039045227836e2 * t3937 * t3915;
    (t12218, t12224, t12225, t12229, t12233, t12235, t12238, t12240)
}
