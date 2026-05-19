//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1006/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1006<F: Float>(t245: F, t5420: F, t712: F, t5427: F, t723: F, t1903: F, t1910: F, t1924: F, t1917: F, t703: F, t17569: F, t17571: F, t17573: F, t17575: F, t17581: F, t17583: F, t18305: F) -> F {
    let t18309 = F::new(0.2e-20) * t712 * t245 * t5420;
    let t18311 = F::new(8.0) / F::new(9.0) * t5427 * t723;
    let t18312 = t1910 * t1903;
    let t18315 = F::new(4.0) / F::new(9.0) * t1924 * t1903;
    let t18318 = F::cast_from(0.5402469135802469136e-1_f64) * t712 * t703 * t1917;
    let t18319 = t17569 + t17571 + t17573 + t17575 + t17581 + F::new(0.14e-19) * t18305 + t18309 + t18311 - F::new(8.0) / F::new(9.0) * t18312 - t18315 - t18318 - t17583;
    t18319
}
