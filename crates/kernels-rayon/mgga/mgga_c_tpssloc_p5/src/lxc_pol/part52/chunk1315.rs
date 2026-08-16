//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1315/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1315(t31069: f64, t4028: f64, t33085: f64, t6525: f64, t1983: f64, t24990: f64, t31047: f64, t26103: f64, t7468: f64, t26003: f64, t6517: f64, t24999: f64, t6535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120029 = t4028 * t31069;
    let t120040 = t33085 * t6525;
    let t120044 = 3.0_f64 * t1983 * t31047 * t24990;
    let t120045 = t26103 * t7468;
    let t120047 = t6517 * t26003;
    let t120049 = t24999 * t6535;
    (t120029, t120040, t120044, t120045, t120047, t120049)
}
