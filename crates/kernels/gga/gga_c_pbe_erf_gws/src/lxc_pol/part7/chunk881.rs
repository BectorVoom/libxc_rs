//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 881/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk881<F: Float>(t1413: F, t1697: F, t1793: F, t4927: F, t639: F, t4923: F, t5129: F, t587: F, t1765: F, t1804: F, t5548: F, t1672: F, t185: F, t1867: F) -> (F, F, F, F) {
    let t16811 = F::new(32.0) / F::new(15.0) * t639 * t4927 * t1793 * t1697 * t1413;
    let t16813 = t587 * t5129 * t4923;
    let t16814 = F::new(64.0) / F::new(45.0) * t16813;
    let t16818 = F::new(32.0) / F::new(15.0) * t587 * t5548 * t1765 * t1804;
    let t16820 = t185 * t1672 * t1867;
    (t16811, t16814, t16818, t16820)
}
