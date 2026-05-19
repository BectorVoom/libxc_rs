//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 962/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk962<F: Float>(t37723: F, t37725: F, t37728: F, t37733: F, t37736: F, t37739: F, t37742: F, t37745: F, t37752: F, t37756: F, t37758: F, t37820: F) -> (F, F) {
    let t40012 = F::cast_from(0.13039253546995884774e1_f64) * t37723 + F::cast_from(0.14224640233086419754e1_f64) * t37725 - F::cast_from(0.17780800291358024693e0_f64) * t37728 - F::cast_from(0.62232801019753086422e0_f64) * t37733 - F::cast_from(0.22226000364197530866e-1_f64) * t37736 - F::cast_from(0.29634667152263374488e-1_f64) * t37739 + F::cast_from(0.22226000364197530865e-1_f64) * t37742 + F::cast_from(0.69147556688614540471e-1_f64) * t37745 + F::cast_from(0.17286889172153635117e0_f64) * t37752 + F::cast_from(0.16669500273148148149e-1_f64) * t37756 - F::cast_from(0.10668480174814814815e1_f64) * t37758;
    let t40033 = F::cast_from(0.4939111192043895748e-1_f64) * t37820;
    (t40012, t40033)
}
