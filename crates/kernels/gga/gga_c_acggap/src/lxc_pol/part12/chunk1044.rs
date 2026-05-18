//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1044/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1044<F: Float>(t30402: F, t30407: F, t30409: F, t513: F, t7447: F, t8637: F, t4578: F, t7450: F, t7815: F, t4483: F, t2030: F, t4582: F) -> (F, F, F, F, F) {
    let t34590 = t30407 * t30402 * t30409 * t513;
    let t34592 = t7447 * t8637;
    let t34595 = t7450 * t7815 * t4578;
    let t34598 = t7450 * t7815 * t4483;
    let t34601 = t2030 * t7815 * t4582;
    (t34590, t34592, t34595, t34598, t34601)
}
