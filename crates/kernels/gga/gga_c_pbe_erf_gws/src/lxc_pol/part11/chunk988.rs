//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 988/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk988<F: Float>(t11159: F, t713: F, t1923: F, t256: F, t3583: F, t10606: F, t723: F, t1903: F, t3584: F, t10610: F, t1918: F, t1617: F, t3603: F) -> (F, F, F, F, F, F) {
    let t34395 = t11159 * t713;
    let t34418 = t3583 * t1923 * t256;
    let t34500 = t10606 * t723;
    let t34538 = t3584 * t1903;
    let t34544 = t10610 * t1918;
    let t34565 = t3603 * t1617;
    (t34395, t34418, t34500, t34538, t34544, t34565)
}
