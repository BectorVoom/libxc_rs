//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1171/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1171<F: Float>(t2156: F, t816: F, t2157: F, t2074: F, t824: F, t821: F, t6184: F, t6217: F, t6538: F, t2313: F, t745: F, t2148: F) -> (F, F, F, F, F, F, F) {
    let t20886 = t816 * t2156;
    let t20887 = t20886 * t2157;
    let t20898 = t824 * t2074;
    let t20899 = t821 * t20898;
    let t20903 = t6217 * t6184;
    let t20904 = F::new(7.0) / F::new(24.0) * t20903;
    let t20905 = t6538 * t6184;
    let t20906 = F::new(7.0) / F::new(24.0) * t20905;
    let t20907 = t2313 * t745;
    let t20912 = t2148 * t2074;
    (t20886, t20887, t20899, t20904, t20906, t20907, t20912)
}
