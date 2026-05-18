//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 830/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk830<F: Float>(t2424: F, t804: F, t810: F, t2051: F, t944: F, t2052: F, t381: F, t321: F, t2074: F, t946: F, t2075: F, t2429: F) -> (F, F, F, F, F, F) {
    let t6850 = t804 * t2424 * t810;
    let t6852 = t2051 * t944;
    let t6854 = F::new(1.0) / t2052 / t381;
    let t6855 = t6852 * t6854;
    let t6856 = t321 * t6855;
    let t6860 = t804 * t946 * t2074;
    let t6863 = t2429 * t2075 * t810;
    (t6850, t6854, t6855, t6856, t6860, t6863)
}
