//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1256/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1256<F: Float>(t11564: F, t11808: F, t11557: F, t11994: F, t13539: F, t2255: F, t2277: F, t3257: F, t3780: F, t45755: F, t45793: F, t49921: F, t49928: F, t49929: F, t49931: F, t49936: F, t49943: F, t9441: F) -> (F, F) {
    let t49945 = t11564 * t11808 / F::new(8.0);
    let t49946 = -t49921 - F::new(7.0) / F::new(384.0) * t2277 * t3257 * t9441 * t11557 * t3780 - t49928 + t49929 + F::new(7.0) / F::new(96.0) * t45755 + t49931 + t49936 + F::new(7.0) / F::new(576.0) * t45793 - t2277 * t2255 * t11994 * t13539 / F::new(256.0) - t49943 - t49945;
    (t49945, t49946)
}
