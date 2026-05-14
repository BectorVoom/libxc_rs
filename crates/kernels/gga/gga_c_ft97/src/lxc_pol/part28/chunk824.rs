//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 824/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk824<F: Float>(t27015: F, t50249: F, t604: F, t6615: F, t1391: F, t9114: F, t1378: F, t526: F, t9439: F, t605: F, t9016: F, t2178: F, t5842: F, t23571: F, t2101: F, t6685: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106565 = t50249 * t27015;
    let t106573 = t604 * t6615;
    let t106619 = t9114 * t1391;
    let t106623 = t526 * t1378;
    let t106698 = t526 * t9439;
    let t106729 = t9016 * t605;
    let t106761 = t2178 * t5842;
    let t106803 = t50249 * t23571;
    let t106875 = t2101 * t6685;
    (t106565, t106573, t106619, t106623, t106698, t106729, t106761, t106803, t106875)
}
