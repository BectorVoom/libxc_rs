//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 859/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk859<F: Float>(t864: F, t879: F, t317: F, t3922: F, t3915: F, t3937: F, t3889: F, t852: F, t3919: F, t3868: F, t1264: F, t449: F, t863: F) -> (F, F, F, F, F, F, F) {
    let t12235 = t864 * t879;
    let t12238 = F::new(0.23707617058567841754e2) * t3922 * t317 * t12235;
    let t12240 = F::new(0.15805078039045227836e2) * t3937 * t3915;
    let t12241 = t852 * t3889;
    let t12243 = t3937 * t3919;
    let t12246 = t3868 * t3919;
    let t12250 = t863 * t449 * t864 * t1264;
    (t12235, t12238, t12240, t12241, t12243, t12246, t12250)
}
