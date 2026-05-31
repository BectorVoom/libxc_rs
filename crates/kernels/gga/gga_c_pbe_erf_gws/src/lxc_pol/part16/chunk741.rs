//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 741/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk741<F: Float>(t1294: F, t174: F, t4715: F, t1258: F, t155: F, t331: F, t434: F, t456: F, t4607: F, t1318: F, t448: F, t75: F) -> (F, F, F, F, F, F) {
    let t4717 = t174 * t4715 * t1294;
    let t4718 = F::cast_from(0.85917146441092277512e0_f64) * t4717;
    let t4719 = t155 * t1258;
    let t4723 = t331 * t434;
    let t4730 = t4607 * t456;
    let t4734 = F::cast_from(1.0_f64) / t1318 / t448;
    let t4735 = t75 * t4734;
    (t4718, t4719, t4723, t4730, t4734, t4735)
}
