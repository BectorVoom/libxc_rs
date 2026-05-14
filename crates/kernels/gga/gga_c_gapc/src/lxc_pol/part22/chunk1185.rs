//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1185/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1185<F: Float>(t34517: F, t34520: F, t34522: F, t34525: F, t34528: F, t34537: F, t34539: F, t34530: F, t34533: F, t34547: F, t36969: F, t34553: F, t34555: F, t34557: F, t34560: F, t34563: F) -> (F, F, F, F, F, F) {
    let t36970 = 0.25301920572916666668e-5 * t34517;
    let t36971 = 0.50603841145833333336e-5 * t34520;
    let t36972 = 0.25301920572916666668e-5 * t34522;
    let t36973 = 0.50603841145833333336e-5 * t34525;
    let t36974 = 0.48917046440972222224e-4 * t34528;
    let t36977 = 0.13111033542209201391e-7 * t34537;
    let t36978 = 0.14068827330203670243e-7 * t34539;
    let t36980 = t36969 + t36970 + t36971 + t36972 - t36973 - t36974 + 0.5691280480400994668e-7 * t34530 + 0.68761854623411138862e-8 * t34533 - t36977 + t36978 + 0.56399158975894962976e-8 * t34547;
    let t36982 = 0.13506074236995523433e-5 * t34553;
    let t36983 = 0.13506074236995523433e-5 * t34555;
    let t36984 = 0.67530371184977617164e-6 * t34557;
    let t36985 = 0.10567613244746075633e-6 * t34560;
    let t36986 = 0.1167337499678099199e-7 * t34563;
    (t36980, t36982, t36983, t36984, t36985, t36986)
}
