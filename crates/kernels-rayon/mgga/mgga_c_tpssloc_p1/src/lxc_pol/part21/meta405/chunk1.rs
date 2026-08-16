//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1891/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1891(t1060: f64, t14595: f64, t4673: f64, t4677: f64, t1625: f64, t3120: f64, t14506: f64, t3199: f64) -> (f64, f64, f64, f64) {
    let t14596 = t14595 * t1060;
    let t14600 = t4677 * t4673;
    let t14605 = t1625 * t3120;
    let t14606 = t14605 * t1060;
    let t14608 = t14506 * t3199;
    (t14596, t14600, t14606, t14608)
}
