//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 836/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk836<F: Float>(t262: F, t45720: F, t8640: F, t34724: F, t9709: F, t558: F, t8704: F, t7198: F, t1614: F, t2347: F, t2286: F, t9087: F, t2412: F, t8587: F, t2191: F, t9795: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t45721 = t262 * t45720;
    let t45722 = t8640 * t45721;
    let t45724 = t34724 * t9709;
    let t45726 = t8704 * t558;
    let t45727 = t262 * t45726;
    let t45728 = t7198 * t45727;
    let t45730 = t2347 * t1614;
    let t45731 = t262 * t45730;
    let t45732 = t7198 * t45731;
    let t45734 = t9087 * t2286;
    let t45736 = t2412 * t8587;
    let t45738 = t2191 * t9795;
    (t45721, t45722, t45724, t45726, t45727, t45728, t45730, t45731, t45732, t45734, t45736, t45738)
}
