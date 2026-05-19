//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1192/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1192<F: Float>(t3444: F, t7211: F, t10743: F, t2549: F, t24908: F, t2562: F, t883: F, t943: F, t10683: F, t7137: F, t10682: F, t2042: F, t2508: F) -> (F, F, F, F, F) {
    let t32116 = t7211 * t3444;
    let t32117 = F::cast_from(0.32043859292259267849e-3_f64) * t32116;
    let t32118 = t2549 * t10743;
    let t32119 = F::cast_from(0.64087718584518535698e-3_f64) * t32118;
    let t32122 = t943 * t2562 * t883 * t24908;
    let t32123 = F::cast_from(0.32043859292259267849e-3_f64) * t32122;
    let t32125 = F::cast_from(0.20508069947045931424e-1_f64) * t7137 * t10683;
    let t32128 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2042 * t10682;
    (t32117, t32119, t32123, t32125, t32128)
}
