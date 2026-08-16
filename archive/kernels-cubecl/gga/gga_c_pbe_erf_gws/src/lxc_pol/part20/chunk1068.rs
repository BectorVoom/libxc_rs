//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1068/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1068<F: Float>(t858: F, t886: F, t9807: F, t884: F, t11974: F, t11976: F, t11977: F, t11979: F, t11983: F, t11986: F, t11989: F, t11991: F, t11996: F, t2277: F, t2343: F, t9142: F, t9601: F, t9632: F) -> (F, F, F) {
    let t12000 = t886 * t858 * t9807;
    let t12002 = t884 * t12000 / F::cast_from(48.0_f64);
    let t12003 = t11974 - t9601 - t9142 - t11976 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t11977 - t11979 - t11983 - t11986 + t11989 + t2343 * t11991 / F::cast_from(192.0_f64) + t2277 * t11996 / F::cast_from(768.0_f64) - t9632 - t12002;
    (t12000, t12002, t12003)
}
