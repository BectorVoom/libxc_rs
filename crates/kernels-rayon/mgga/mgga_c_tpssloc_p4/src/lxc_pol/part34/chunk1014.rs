//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1014/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1014(t2752: f64, t28: f64, t22468: f64, t2094: f64, t531: f64, t7025: f64, t9239: f64, t33: f64, t625: f64, t2240: f64, t240: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23788 = t2752 * t28;
    let t23912 = 22.0_f64 / 9.0_f64 * t22468;
    let t23957 = t531 * t2094;
    let t23963 = t9239 * t7025;
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23992 = t240 * t67;
    (t23788, t23912, t23957, t23963, t23966, t23967, t23992)
}
