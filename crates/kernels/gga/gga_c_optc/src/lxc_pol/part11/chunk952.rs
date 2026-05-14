//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 952/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk952<F: Float>(t23685: F, t346: F, t349: F, t8343: F, t23682: F, t2471: F, t2475: F, t214: F, t211: F, t217: F, t22502: F, t2528: F, t2414: F, t216: F, t2417: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23686 = 0.20068888888888888889e-1 * t23685;
    let t23708 = t346 / t8343 / t349;
    let t23769 = 0.75383950617283950617e4 * t23682;
    let t23770 = 0.12819753086419753086e4 * t23685;
    let t23800 = t2471 * t2471;
    let t23801 = 1.0 / t23800;
    let t23803 = t2475 * t2475;
    let t23804 = 1.0 / t23803;
    let t23844 = f64::powf(t214, -0.25e1);
    let t23860 = 280.0 / 81.0 * t23682;
    let t23913 = 1.0 / t217 / t22502 / t211 / 96.0;
    let t23926 = 0.31310740740740740741e1 * t23682;
    let t23927 = 0.13490888888888888889e1 * t23685;
    let t24021 = 1.0 / t2471 / t2528;
    let t24287 = 0.31003950617283950618e1 * t23682;
    let t24288 = 0.13388493827160493828e1 * t23685;
    let t24300 = t2414 * t2414;
    let t24302 = t216 / t24300;
    let t24304 = t2417 * t2417;
    (t23686, t23708, t23769, t23770, t23801, t23804, t23844, t23860, t23913, t23926, t23927, t24021, t24287, t24288, t24302, t24304)
}
