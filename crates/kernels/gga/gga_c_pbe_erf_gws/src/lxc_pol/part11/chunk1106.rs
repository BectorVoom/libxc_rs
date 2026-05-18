//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1106/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1106<F: Float>(t40718: F, t1017: F, t1885: F, t40571: F, t587: F, t17182: F, t17183: F, t47391: F, t10383: F, t3443: F, t1620: F, t1621: F, t31503: F, t3390: F) -> (F, F, F, F, F) {
    let t47707 = F::new(32.0) / F::new(27.0) * t40718;
    let t47711 = F::new(16.0) / F::new(15.0) * t587 * t1885 * t40571 * t1017;
    let t47715 = F::new(352.0) / F::new(243.0) * t587 * t17182 * t17183 * t47391;
    let t47719 = F::new(24.0) / F::new(5.0) * t587 * t1885 * t10383 * t3443;
    let t47723 = F::new(16.0) / F::new(5.0) * t1620 * t1621 * t31503 * t3390;
    (t47707, t47711, t47715, t47719, t47723)
}
