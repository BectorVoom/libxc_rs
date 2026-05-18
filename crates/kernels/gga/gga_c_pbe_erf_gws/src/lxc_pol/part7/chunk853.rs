//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 853/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk853<F: Float>(t20: F, t2004: F, t5450: F, t5942: F, t5953: F, t156: F, t5926: F, t670: F, t1999: F, t542: F, t1673: F, t1775: F) -> (F, F, F, F, F) {
    let t16492 = t5450 * t20 * t2004;
    let t16494 = t5953 * t5942;
    let t16498 = F::new(0.43284165449459373508e0) * t670 * t156 * t5926;
    let t16501 = F::new(0.38474813732852776452e0) * t670 * t542 * t1999;
    let t16502 = t1775 * t1673;
    (t16492, t16494, t16498, t16501, t16502)
}
