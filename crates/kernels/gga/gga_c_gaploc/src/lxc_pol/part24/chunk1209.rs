//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1209/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1209<F: Float>(t1853: F, t3432: F, t3444: F, t7211: F, t10743: F, t2549: F, t24908: F, t2562: F, t883: F, t943: F, t10683: F, t7137: F) -> (F, F, F, F, F) {
    let t32112 = t3432 * t1853;
    let t32116 = t7211 * t3444;
    let t32117 = F::new(0.32043859292259267849e-3) * t32116;
    let t32118 = t2549 * t10743;
    let t32119 = F::new(0.64087718584518535698e-3) * t32118;
    let t32122 = t943 * t2562 * t883 * t24908;
    let t32123 = F::new(0.32043859292259267849e-3) * t32122;
    let t32125 = F::new(0.20508069947045931424e-1) * t7137 * t10683;
    (t32112, t32117, t32119, t32123, t32125)
}
