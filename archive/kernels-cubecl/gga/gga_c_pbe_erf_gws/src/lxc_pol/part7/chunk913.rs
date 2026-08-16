//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 913/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk913<F: Float>(t17174: F, t16746: F, t587: F, t590: F, t591: F, t1663: F, t187: F, t22: F, t16740: F, t197: F, t16669: F, t2620: F, t592: F) -> (F, F, F, F) {
    let t17175 = F::cast_from(256.0_f64) / F::cast_from(243.0_f64) * t17174;
    let t17179 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t587 * t590 * t591 * t16746;
    let t17182 = t22 / t187 / t1663;
    let t17183 = t197 * t16740;
    let t17187 = F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t587 * t17182 * t17183 * t16669;
    let t17188 = t2620 * t592;
    (t17175, t17179, t17187, t17188)
}
