//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 816/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk816<F: Float>(t30594: F, t584: F, t1072: F, t167: F, t7322: F, t145: F, t301: F, t721: F, t174: F, t372: F, t7859: F, t2016: F, t7596: F, t7343: F, t7433: F, t30105: F, t7348: F) -> (F, F, F, F, F, F, F) {
    let t30595 = t30594 * t584;
    let t30598 = t7322 * t167 * t1072;
    let t30601 = t30598 * t145 * t301 * t721;
    let t30605 = t7859 * t174 * t372 * t721;
    let t30607 = t2016 * t7596;
    let t30611 = t7433 * t7343;
    let t30613 = t30105 * t7348;
    (t30595, t30598, t30601, t30605, t30607, t30611, t30613)
}
