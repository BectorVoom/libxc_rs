//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk966;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk967;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta185<F: Float>(t1489: F, t2563: F, t131: F, t2570: F, t205: F, t1484: F, t213: F, t221: F, t776: F, t118: F, t794: F, t2576: F, t210: F, t214: F, t4119: F, t2562: F, t2564: F, t2569: F, t2579: F, t2590: F, t787: F, t252: F, t1492: F, t852: F, t1493: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4124, t4126, t4127, t4128, t4130, t4134, t4135) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk966::<F>(t1489, t2563, t131, t2570, t205, t1484, t213, t221, t776, t118, t794, t2576);
        let (t4138, t4142) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk967::<F>(t210, t214, t4119, t2562, t2564, t2569, t2579, t2590, t4124, t4127, t4130, t4135, t787);
        let (t4143, t4145, t4147) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk968::<F>(t252, t4142, t1492, t852, t1493, t225);
    (t4126, t4127, t4128, t4130, t4134, t4138, t4142, t4143, t4145, t4147)
}
