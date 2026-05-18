//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 718/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk718<F: Float>(t2704: F, t2718: F, t7: F, t226: F, t225: F, t4562: F, t1989: F, t679: F, t1986: F, t666: F, t231: F, t4986: F, t4990: F, t4994: F, t4997: F, t5000: F, t5007: F, t5012: F, t5017: F, t5021: F, t5101: F, t5104: F, t5107: F, t5115: F) -> (F, F, F, F, F) {
    let t5902 = F::new(0.12833333333333333333e1) * t2704 - F::new(20.0) / F::new(27.0) * t2718;
    let t5903 = t5902 * M_PI;
    let t5904 = t5903 * t7;
    let t5906 = F::new(4.0) / F::new(3.0) * t226 * t5904;
    let t5907 = t4562 * t225;
    let t5910 = t1989 * t679;
    let t5912 = t666 * t1986;
    let t5914 = t4986 - t4990 - t4994 - t4997 - t5000 - t5007 - t5012 - t5017 - t5021 - t5101 + t5104 + t5906 + F::new(4.0) / F::new(3.0) * t5907 * t231 + F::new(4.0) * t5910 + F::new(4.0) * t5912 + t5107 - t5115;
    (t5902, t5903, t5904, t5907, t5914)
}
