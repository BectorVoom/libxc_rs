//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 624/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk624<F: Float>(t4862: F, t88: F, t4831: F, t4833: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4852: F, t4854: F, t4856: F, t4858: F, t4861: F) -> (F, F, F) {
    let t4863 = t4862 * t88;
    let t4864 = F::new(120.0) * t4863;
    let t4865 = t4831 + t4833 - t4837 - t4840 - t4843 + t4846 + t4849 + t4852 - t4854 + t4856 - t4858 + t4861 - t4864;
    (t4863, t4864, t4865)
}
