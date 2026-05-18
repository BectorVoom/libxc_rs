//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 862/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk862<F: Float>(t11514: F, t2345: F, t3814: F, t13171: F, t823: F, t850: F, t852: F, t860: F, t1076: F, t1109: F, t2255: F, t3258: F) -> (F, F, F, F, F, F) {
    let t13531 = t2345 * t11514 * t3814;
    let t13534 = t13171 * t823;
    let t13536 = t850 * t13534 * t852;
    let t13538 = t13536 * t860 / F::new(96.0);
    let t13539 = t1076 * t1109;
    let t13541 = t2255 * t3258 * t13539;
    (t13531, t13534, t13536, t13538, t13539, t13541)
}
