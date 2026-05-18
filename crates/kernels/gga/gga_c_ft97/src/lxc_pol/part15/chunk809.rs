//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 809/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk809<F: Float>(t21837: F, t898: F, t900: F, t4357: F, t5468: F, t20489: F, t231: F, t893: F, t1268: F, t5457: F, t10904: F, t10915: F, t10916: F, t21181: F) -> (F, F, F, F, F, F) {
    let t21839 = t898 * t900 * t21837;
    let t21843 = t898 * t4357 * t5468;
    let t21847 = t231 * t893 * t20489;
    let t21850 = t5457 * t1268;
    let t21852 = t898 * t10904 * t21850;
    let t21856 = t10915 * t10916 * t21181;
    (t21839, t21843, t21847, t21850, t21852, t21856)
}
