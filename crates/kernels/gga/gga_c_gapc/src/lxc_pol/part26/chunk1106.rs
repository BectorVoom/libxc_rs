//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1106/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1106<F: Float>(t11745: F, t18331: F, t11387: F, t7204: F, t7557: F, t11483: F, t11749: F, t2787: F, t33701: F, t33704: F, t33707: F, t33710: F, t33714: F, t33717: F, t33719: F, t33726: F) -> F {
    let t33728 = t18331 * t11745;
    let t33731 = t7204 * t11387 * t7557;
    let t33734 = t2787 * t11483 * t11749;
    let t33736 = -F::cast_from(0.21720231316129303386e-4_f64) * t33701 - F::cast_from(0.21720231316129303386e-4_f64) * t33704 - F::cast_from(0.10860115658064651693e-4_f64) * t33707 - F::cast_from(0.20611878024038059902e-5_f64) * t33710 + F::cast_from(0.36647919126739670507e-5_f64) * t33714 - F::cast_from(0.36872409820556640627e-8_f64) * t33717 + F::cast_from(0.63252766927083333336e-6_f64) * t33719 + F::cast_from(0.20240885416666666668e-4_f64) * t33726 - F::cast_from(0.5686343261418565457e-6_f64) * t33728 - F::cast_from(0.5686343261418565457e-6_f64) * t33731 + F::cast_from(0.2318836277704281739e-4_f64) * t33734;
    t33736
}
