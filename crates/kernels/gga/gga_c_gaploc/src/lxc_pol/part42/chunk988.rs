//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 988/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk988<F: Float>(t12255: F, t1841: F, t1897: F, t2508: F, t2580: F, t2958: F, t38912: F, t44956: F, t44960: F, t44963: F, t44972: F, t44976: F, t44990: F, t44992: F, t44994: F, t44998: F, t45000: F, t45009: F, t45010: F, t45015: F, t47687: F, t47690: F, t50002: F, t7289: F, t8682: F) -> F {
    let t50435 = -t44956 + t44960 + t44963 + t44972 + t44976 - t44990 - t44992 - t44994 - t44998 + t45000 - F::new(0.10766736722199113997e0) * t2508 * t12255 * t8682 + F::new(0.1281754371690370714e-2) * t47687 + F::new(0.1281754371690370714e-2) * t47690 - F::new(0.34180116578409885704e-2) * t1841 * t7289 * t50002 + t45009 - F::new(0.96131577876777803547e-3) * t45010 - t45015 - F::new(0.30762104920568897134e-1) * t1897 * t2580 * t2958 * t38912;
    t50435
}
