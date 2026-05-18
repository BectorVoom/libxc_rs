//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1412/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1412<F: Float>(t15174: F, t5186: F, t8686: F, t1102: F, t26224: F, t58311: F, t8700: F, t14871: F, t15374: F, t17529: F, t4305: F, t3061: F, t8697: F) -> (F, F, F, F, F) {
    let t59205 = F::new(0.3103500882342370105e4) * t8686 * t15174 * t5186;
    let t59209 = F::new(0.12304676425209353917e5) * t1102 * t26224 * t58311 * t8700;
    let t59212 = F::new(0.62336721237753107879e3) * t1102 * t14871 * t15374;
    let t59214 = F::new(0.14035736153892489771e2) * t4305 * t17529;
    let t59218 = F::new(0.6233672123775310788e3) * t1102 * t8697 * t58311 * t3061;
    (t59205, t59209, t59212, t59214, t59218)
}
