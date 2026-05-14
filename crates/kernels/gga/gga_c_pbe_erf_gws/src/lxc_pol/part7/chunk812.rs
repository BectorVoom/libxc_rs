//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 812/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk812<F: Float>(t16797: F, t4929: F, t639: F, t5109: F, t642: F, t422: F, t5111: F, t626: F, t1413: F, t1697: F, t1793: F, t4927: F, t4923: F, t5129: F, t587: F, t1765: F, t1804: F, t5548: F) -> (F, F, F, F, F) {
    let t16799 = t639 * t16797 * t4929;
    let t16800 = 64.0 / 45.0 * t16799;
    let t16801 = t642 * t5109;
    let t16806 = 32.0 / 15.0 * t639 * t16801 * t5111 * t626 * t422;
    let t16811 = 32.0 / 15.0 * t639 * t4927 * t1793 * t1697 * t1413;
    let t16813 = t587 * t5129 * t4923;
    let t16814 = 64.0 / 45.0 * t16813;
    let t16818 = 32.0 / 15.0 * t587 * t5548 * t1765 * t1804;
    (t16800, t16806, t16811, t16814, t16818)
}
