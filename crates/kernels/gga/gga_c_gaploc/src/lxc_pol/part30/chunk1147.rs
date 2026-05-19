//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1147/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1147<F: Float>(t2464: F, t2476: F, t9440: F, t2349: F, t4752: F, t20671: F, t20688: F, t20700: F, t1538: F, t20539: F, t21283: F, t883: F) -> (F, F, F, F) {
    let t30927 = t2476 * t2464 * t9440;
    let t30949 = t4752 * t2349;
    let t31018 = F::cast_from(0.17041300423964777634e0_f64) * t20688 * t20671 * t20700;
    let t31021 = t21283 * t1538 * t883 * t20539;
    (t30927, t30949, t31018, t31021)
}
