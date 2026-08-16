//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1163/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1163(t2537: f64, t7607: f64, t2539: f64, t2165: f64, t26411: f64, t26422: f64, t26556: f64, t2771: f64, t2789: f64, t36436: f64, t36513: f64, t7660: f64, t7669: f64, t899: f64, t9007: f64, t9018: f64, t9021: f64, t906: f64, t91885: f64, t91895: f64, t91901: f64, t92134: f64, t92149: f64) -> (f64, f64) {
    let t92155 = t7607 * t2537;
    let t92157 = 6.0_f64 * t92155 * t2539;
    let t92158 = 6.0_f64 * t36436 * t7660 - 3.0_f64 * t9007 * t7669 + t91885 - 3.0_f64 * t26422 * t2789 + 24.0_f64 * t36513 * t2165 * t9018 - t91895 + t91901 + 6.0_f64 * t26411 * t9021 - t899 * (t92134 + t92149) + 6.0_f64 * t2771 * t26556 * t906 - t92157;
    (t92157, t92158)
}
