//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2250/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2250(t5587: f64, t81886: f64, t23041: f64, t5619: f64, t16753: f64, t6605: f64, t815: f64, t16928: f64, t25084: f64, t16851: f64, t221: f64, t87420: f64) -> (f64, f64, f64, f64, f64) {
    let t98796 = t81886 * t5587;
    let t98798 = t23041 * t5619;
    let t98801 = t6605 * t815 * t16753;
    let t98803 = t25084 * t16928;
    let t98808 = t87420 * t221 * t16851;
    (t98796, t98798, t98801, t98803, t98808)
}
