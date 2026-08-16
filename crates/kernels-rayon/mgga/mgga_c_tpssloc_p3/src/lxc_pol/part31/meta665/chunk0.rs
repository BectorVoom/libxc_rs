//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1954/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1954(t16944: f64, t25891: f64, t25927: f64, t98111: f64, t1649: f64, t4119: f64, t23788: f64, t67123: f64, t1081: f64, t5660: f64, t5544: f64, t16662: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100708 = t25891 * t16944;
    let t100713 = t25927 * t98111;
    let t100718 = t1649 * t4119;
    let t100731 = t23788 * t67123;
    let t100734 = t1081 * t5660;
    let t100743 = t1081 * t5544;
    let t100747 = t28 * t16662;
    (t100708, t100713, t100718, t100731, t100734, t100743, t100747)
}
