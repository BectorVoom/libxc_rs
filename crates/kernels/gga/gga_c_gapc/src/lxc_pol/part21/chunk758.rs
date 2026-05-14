//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 758/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk758<F: Float>(t9709: F, t9710: F, t8986: F, t961: F, t7967: F, t1072: F, t1074: F, t2387: F, t1069: F, t2489: F, t102: F, t818: F, t329: F, t3407: F, t3403: F, t9680: F, t9683: F, t9686: F, t9690: F, t9693: F, t9698: F, t9704: F, t9707: F) -> (F, F, F) {
    let t9711 = t9709 * t9710;
    let t9713 = t8986 * t961;
    let t9714 = t7967 * t9713;
    let t9717 = t2387 * t1072 * t1074;
    let t9719 = t1069 * t2489;
    let t9721 = t102 * t818;
    let t9722 = t9721 * t329;
    let t9723 = t9722 * t3407;
    let t9724 = t3403 * t9723;
    let t9726 = -0.36954560225358884233e-5 * t9680 + 0.7588373973867992891e-7 * t9683 - 0.13492128925537291361e-6 * t9686 - 0.25745714186718600948e-5 * t9690 + 0.2318836277704281739e-4 * t9693 - 0.37545833188964626383e-6 * t9698 - 0.33199136135672468897e-7 * t9704 + 0.59028064049225649701e-7 * t9707 - 0.93789165502563894766e-9 * t9711 + 0.12647289956446654818e-8 * t9714 + 0.50602213541666666669e-5 * t9717 + 0.13900948042322754167e-2 * t9719 + 0.84410248952307505288e-7 * t9724;
    (t9722, t9723, t9726)
}
