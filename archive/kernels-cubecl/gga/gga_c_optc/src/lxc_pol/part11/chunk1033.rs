//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1033/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1033<F: Float>(t23685: F, t346: F, t349: F, t8343: F, t23682: F, t2471: F, t2475: F, t214: F, t211: F, t217: F, t22502: F, t2528: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23686 = F::cast_from(0.20068888888888888889e-1_f64) * t23685;
    let t23708 = t346 / t8343 / t349;
    let t23769 = F::cast_from(0.75383950617283950617e4_f64) * t23682;
    let t23770 = F::cast_from(0.12819753086419753086e4_f64) * t23685;
    let t23800 = t2471 * t2471;
    let t23801 = F::cast_from(1.0_f64) / t23800;
    let t23803 = t2475 * t2475;
    let t23804 = F::cast_from(1.0_f64) / t23803;
    let t23844 = F::powf(t214, -F::cast_from(0.25e1_f64));
    let t23860 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t23682;
    let t23913 = F::cast_from(1.0_f64) / t217 / t22502 / t211 / F::cast_from(96.0_f64);
    let t23926 = F::cast_from(0.31310740740740740741e1_f64) * t23682;
    let t23927 = F::cast_from(0.13490888888888888889e1_f64) * t23685;
    let t24021 = F::cast_from(1.0_f64) / t2471 / t2528;
    (t23686, t23708, t23769, t23770, t23801, t23804, t23844, t23860, t23913, t23926, t23927, t24021)
}
