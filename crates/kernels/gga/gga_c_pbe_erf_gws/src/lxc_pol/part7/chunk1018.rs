//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1018/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1018<F: Float>(t461: F, t4859: F, t14: F, t2: F, t41: F, t174: F, t6045: F, t413: F, t4517: F, t366: F, t799: F, t1236: F) -> (F, F, F, F, F, F) {
    let t18478 = t4859 * t461;
    let t18479 = F::new(960.0) * t18478;
    let t18483 = F::new(1.0) / t14 / t2 / t41 / F::new(48.0);
    let t18486 = t18483 * t2 * t6045 * t174;
    let t18488 = t4517 * t413;
    let t18490 = t799 * t366;
    let t18491 = t1236 * t18490;
    (t18479, t18483, t18486, t18488, t18490, t18491)
}
