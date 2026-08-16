//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1073/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1073(t1760: f64, t6267: f64, t3598: f64, t6243: f64, t11606: f64, t11764: f64, t20234: f64, t974: f64, t1743: f64, t6169: f64, t11487: f64, t14766: f64, t18494: f64, t18505: f64, t18512: f64, t21747: f64, t21751: f64, t21789: f64, t21792: f64, t21795: f64, t21802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22003 = t1760 * t6267;
    let t22004 = t3598 * t22003;
    let t22007 = t6243 * t1760;
    let t22008 = t11606 * t22007;
    let t22011 = t11764 * t20234;
    let t22012 = t974 * t22011;
    let t22015 = t6169 * t1743;
    let t22032 = t11487 - 5.0_f64 / 9.0_f64 * t14766 - t18494 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t18505 + t18512 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t21802 + t21789 / 3.0_f64 + t21747 / 6.0_f64 - t21792 - t21751 - t21795 / 6.0_f64;
    (t22004, t22008, t22011, t22012, t22015, t22032)
}
