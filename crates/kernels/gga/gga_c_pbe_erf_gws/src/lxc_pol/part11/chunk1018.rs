//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1018/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1018<F: Float>(t42175: F, t33298: F, t25514: F, t34565: F, t48373: F, t48377: F, t48380: F, t48381: F, t48382: F, t48387: F, t48392: F, t1017: F, t10843: F, t12480: F, t12635: F, t12794: F, t12810: F, t1809: F, t1820: F, t1821: F, t1827: F, t2615: F, t30740: F, t32629: F, t3342: F, t3415: F, t42187: F, t42189: F, t42191: F, t42204: F, t47983: F, t587: F, t639: F, t7130: F) -> (F, F, F, F) {
    let t48393 = 64.0 / 45.0 * t42175;
    let t48394 = 32.0 / 135.0 * t33298;
    let t48395 = t48373 + t48377 + t48380 - t48381 + t48382 + 4.0 / 45.0 * t34565 + t48387 - 0.26596355555555555555e0 * t25514 - t48392 + t48393 - t48394;
    let t48423 = 32.0 / 15.0 * t42187 + 32.0 / 27.0 * t42189 + 64.0 / 45.0 * t42191 - 32.0 / 15.0 * t1820 * t1821 * t30740 * t3342 - 64.0 / 15.0 * t7130 * t12810 + 16.0 / 15.0 * t587 * t1827 * t32629 * t3342 - 32.0 / 15.0 * t10843 * t3415 + 16.0 / 5.0 * t639 * t1809 * t47983 - 16.0 / 15.0 * t2615 * t12635 - 16.0 / 45.0 * t587 * t1827 * t12480 * t1017 - 32.0 / 15.0 * t2615 * t12794 + 64.0 / 45.0 * t42204;
    (t48393, t48394, t48395, t48423)
}
