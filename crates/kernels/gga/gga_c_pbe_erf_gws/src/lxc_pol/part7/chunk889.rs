//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 889/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk889<F: Float>(t16895: F, t1827: F, t4967: F, t587: F, t610: F, t1627: F, t5149: F, t16874: F, t16876: F, t16877: F, t16881: F, t16884: F, t16889: F, t16891: F, t16893: F) -> (F, F, F, F) {
    let t16896 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t16895;
    let t16900 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t587 * t1827 * t4967 * t610;
    let t16902 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t1627 * t5149;
    let t16903 = -t16874 - t16876 + F::cast_from(0.44134814814814814813e-2_f64) * t16877 - t16881 - t16884 + t16889 - t16891 - t16893 + t16896 - t16900 - t16902;
    (t16896, t16900, t16902, t16903)
}
