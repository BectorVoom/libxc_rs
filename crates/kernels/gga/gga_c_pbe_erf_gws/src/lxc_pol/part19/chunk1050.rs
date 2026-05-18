//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1050/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1050<F: Float>(t2133: F, t3916: F, t2138: F, t3111: F, t3763: F, t2255: F, t1109: F, t745: F, t3258: F, t3717: F, t5: F, t337: F) -> (F, F, F, F, F, F, F) {
    let t11794 = t3916 * t2133;
    let t11796 = t11794 * t2138 / F::new(96.0);
    let t11797 = t3111 * t3763;
    let t11798 = t2255 * t11797;
    let t11801 = t745 * t1109;
    let t11803 = t2255 * t3258 * t11801;
    let t11806 = t5 * t3717;
    let t11807 = t337 * t11806;
    (t11794, t11796, t11797, t11798, t11803, t11806, t11807)
}
