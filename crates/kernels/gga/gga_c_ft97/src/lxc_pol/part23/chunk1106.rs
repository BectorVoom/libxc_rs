//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1106/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1106<F: Float>(t38953: F, t6275: F, t6355: F, t8232: F, t6260: F, t870: F, t6284: F, t6293: F, t25271: F, t56110: F, t6280: F, t6289: F, t6347: F, t848: F, t10491: F, t1495: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98823 = t38953 * t6275;
    let t98850 = t8232 * t6355;
    let t98899 = t870 * t6260;
    let t98940 = t8232 * t6284;
    let t98942 = t8232 * t6293;
    let t98966 = t56110 * t25271;
    let t99030 = t8232 * t6280;
    let t99032 = t8232 * t6289;
    let t99034 = t848 * t6347;
    let t99098 = t10491 * t1495;
    (t98823, t98850, t98899, t98940, t98942, t98966, t99030, t99032, t99034, t99098)
}
