//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1401/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1401<F: Float>(t34517: F, t34520: F, t34522: F, t34525: F, t34528: F, t34537: F, t34539: F, t34530: F, t34533: F, t34547: F, t36969: F, t34553: F) -> (F, F) {
    let t36970 = F::cast_from(0.25301920572916666668e-5_f64) * t34517;
    let t36971 = F::cast_from(0.50603841145833333336e-5_f64) * t34520;
    let t36972 = F::cast_from(0.25301920572916666668e-5_f64) * t34522;
    let t36973 = F::cast_from(0.50603841145833333336e-5_f64) * t34525;
    let t36974 = F::cast_from(0.48917046440972222224e-4_f64) * t34528;
    let t36977 = F::cast_from(0.13111033542209201391e-7_f64) * t34537;
    let t36978 = F::cast_from(0.14068827330203670243e-7_f64) * t34539;
    let t36980 = t36969 + t36970 + t36971 + t36972 - t36973 - t36974 + F::cast_from(0.5691280480400994668e-7_f64) * t34530 + F::cast_from(0.68761854623411138862e-8_f64) * t34533 - t36977 + t36978 + F::cast_from(0.56399158975894962976e-8_f64) * t34547;
    let t36982 = F::cast_from(0.13506074236995523433e-5_f64) * t34553;
    (t36980, t36982)
}
