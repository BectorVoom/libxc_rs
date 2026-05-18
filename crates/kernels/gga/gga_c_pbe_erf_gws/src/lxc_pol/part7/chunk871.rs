//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 871/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk871<F: Float>(t11: F, t16699: F, t571: F, t174: F, t177: F, t2200: F, t395: F, t4968: F, t4973: F, t16672: F, t16677: F, t16682: F, t16686: F, t16690: F, t16693: F, t16697: F) -> (F, F, F, F, F) {
    let t16701 = t11 * t571 * t16699;
    let t16704 = t174 * t2200 * t177;
    let t16705 = F::new(0.19591358024691358025e-1) * t16704;
    let t16706 = t395 * t4968;
    let t16708 = t395 * t4973;
    let t16710 = F::new(0.45340000000000000001e-1) * t16672 - F::new(0.45340000000000000002e-1) * t16677 + F::new(0.37783333333333333335e-2) * t16682 + F::new(0.5037777777777777778e-2) * t16686 - F::new(0.4534e-1) * t16690 + F::new(0.6801e-1) * t16693 - F::new(0.11335e-1) * t16697 - F::new(0.15113333333333333333e-1) * t16701 - t16705 - F::new(0.15113333333333333333e-1) * t16706 + F::new(0.15113333333333333333e-1) * t16708;
    (t16701, t16704, t16706, t16708, t16710)
}
