//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 478/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk478<F: Float>(t435: F, t619: F, t2941: F, t1936: F, t1423: F, t522: F, t1006: F, t1033: F, t6: F, t101: F, t1459: F, t1464: F, t520: F, t1005: F, t1599: F, t1603: F, t2937: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2942 = t435 * t619;
    let t2943 = t2941 * t2942;
    let t2945 = t1936 * t619;
    let t2946 = t2941 * t2945;
    let t2948 = t1423 * t522;
    let t2949 = t1006 * t2948;
    let t2951 = t6 * t1033;
    let t2952 = t2951 * t101;
    let t2953 = t2952 * t1459;
    let t2954 = t520 * t1464;
    let t2955 = t2953 * t2954;
    let t2957 = t1005 * t1599;
    let t2958 = t2937 * t1603;
    (t2942, t2943, t2945, t2946, t2948, t2949, t2951, t2952, t2953, t2954, t2955, t2957, t2958)
}
