//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 741/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk741<F: Float>(t2092: F, t2096: F, t4828: F, t4831: F, t4833: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4852: F, t4854: F, t4856: F, t4858: F, t4861: F, t4864: F) -> (F, F) {
    let t6080 = t2092 * t2096;
    let t6081 = F::cast_from(0.20538164420033333334e1_f64) * t6080;
    let t6082 = -t4828 + t4831 + t4833 - t4837 - t4840 - t4843 + t4846 + t4849 + t4852 - t4854 + t6081 + t4856 - t4858 + t4861 - t4864;
    (t6080, t6082)
}
