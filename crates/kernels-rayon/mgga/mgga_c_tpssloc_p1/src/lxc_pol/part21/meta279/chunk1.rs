//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1561/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1561(t10021: f64, t241: f64, t244: f64, t248: f64, t238: f64, t154: f64, t9569: f64, t222: f64, t2606: f64, t9573: f64, t805: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = 595.0_f64 / 10368.0_f64 * t238 * t10024;
    let t10027 = t9569 * t154;
    let t10029 = 455.0_f64 / 1296.0_f64 * t10027 * t222;
    let t10030 = t9573 * t2606;
    let t10036 = t9541 * t805;
    (t10022, t10024, t10026, t10027, t10029, t10030, t10036)
}
