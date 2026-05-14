//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 952/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk952<F: Float>(t4458: F, t5630: F, t1902: F, t103: F, t29726: F, t82: F, t11863: F, t29641: F, t4611: F, t5691: F, t8557: F, t26267: F, t925: F, t1909: F, t11906: F, t6534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29931 = t5630 * t4458;
    let t29932 = t1902 * t29931;
    let t29936 = t82 * t29726 * t103;
    let t29940 = t11863 * t29641;
    let t29943 = t5691 * t4611;
    let t29944 = t8557 * t29943;
    let t29947 = t26267 * t925;
    let t29948 = t1909 * t29947;
    let t29951 = t11906 * t6534;
    (t29931, t29932, t29936, t29940, t29943, t29944, t29947, t29948, t29951)
}
