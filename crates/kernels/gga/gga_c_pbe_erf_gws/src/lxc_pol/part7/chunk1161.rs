//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1161/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1161<F: Float>(t20732: F, t2250: F, t2182: F, t20495: F, t824: F, t2271: F, t6670: F, t822: F, t6674: F, t20206: F, t2407: F, t858: F, param_a_c: F) -> (F, F, F, F, F) {
    let t20733 = t2250 * t20732;
    let t20734 = t2182 * param_a_c;
    let t20739 = t824 * t20495;
    let t20743 = t2271 * t6670;
    let t20744 = t822 * t20743;
    let t20746 = t20744 * t6674 / F::new(4.0);
    let t20748 = t2407 * t858 * t20206;
    (t20733, t20734, t20739, t20746, t20748)
}
