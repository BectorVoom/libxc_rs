//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 947/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk947(t1614: f64, t2347: f64, t262: f64, t7198: f64, t2286: f64, t9087: f64, t2412: f64, t8587: f64, t2191: f64, t9795: f64, t1986: f64, t6590: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45730 = t2347 * t1614;
    let t45731 = t262 * t45730;
    let t45732 = t7198 * t45731;
    let t45734 = t9087 * t2286;
    let t45736 = t2412 * t8587;
    let t45738 = t2191 * t9795;
    let t45742 = t675 * t1986 * t6590;
    (t45730, t45731, t45732, t45734, t45736, t45738, t45742)
}
