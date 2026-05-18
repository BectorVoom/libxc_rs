//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 884/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk884<F: Float>(t198: F, t7776: F, t185: F, t1893: F, t5470: F, t1627: F, t5005: F, t1624: F, t16649: F, t1820: F, t5018: F, t5308: F) -> (F, F, F, F, F) {
    let t16843 = t7776 * t198;
    let t16845 = F::new(112.0) / F::new(1215.0) * t185 * t16843;
    let t16847 = F::new(16.0) / F::new(15.0) * t5470 * t1893;
    let t16849 = F::new(32.0) / F::new(9.0) * t1627 * t5005;
    let t16851 = F::new(16.0) / F::new(5.0) * t16649 * t1624;
    let t16853 = t1820 * t5018 * t5308;
    (t16845, t16847, t16849, t16851, t16853)
}
