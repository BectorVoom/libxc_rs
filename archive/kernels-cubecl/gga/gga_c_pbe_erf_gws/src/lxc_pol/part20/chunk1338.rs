//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1338/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1338<F: Float>(t11966: F, t14011: F, t12036: F, t4023: F, t11697: F, t14101: F, t12074: F, t14567: F, t11794: F, t14069: F, t14079: F, t3857: F) -> (F, F, F, F, F, F) {
    let t57094 = t14011 * t11966;
    let t57096 = t12036 * t4023;
    let t57098 = t14101 * t11697;
    let t57100 = t12074 * t14567;
    let t57102 = t11794 * t14069;
    let t57104 = t14079 * t3857;
    (t57094, t57096, t57098, t57100, t57102, t57104)
}
