//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1185/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185(t3787: f64, t3879: f64, t12248: f64, t1372: f64, t12169: f64, t12171: f64, t12178: f64, t12244: f64, t12255: f64, t12259: f64, t12260: f64, t12435: f64, t1332: f64, t1336: f64, t1352: f64, t22694: f64, t3773: f64, t3777: f64, t3851: f64, t3856: f64, t3901: f64, t3909: f64, t40453: f64, t40475: f64, t5344: f64, t544: f64, t553: f64) -> (f64, f64, f64) {
    let t40486 = t3787 * t3879;
    let t40492 = t12248 * t1372;
    let t40524 = 24.0_f64 * t12171 * t12255 * t1336 - 4.0_f64 * t12178 * t1336 * t3901 - 6.0_f64 * t12259 * t1336 * t3856 - 4.0_f64 * t1352 * t40475 * t5344 - 12.0_f64 * t22694 * t3851 * t5344 + t40453 * t544 * t553 - 4.0_f64 * t12169 * t3777 - 12.0_f64 * t12244 * t3777 - 12.0_f64 * t12260 * t3777 + 4.0_f64 * t12435 * t1332 + 6.0_f64 * t3773 * t3909;
    (t40486, t40492, t40524)
}
