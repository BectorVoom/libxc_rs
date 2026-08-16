//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1411/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1411(t3231: f64, t868: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t23781: f64, t23792: f64, t23796: f64, t23807: f64, t23810: f64, t2522: f64, t28: f64, t4314: f64, t6666: f64, t6670: f64, t81539: f64, t82308: f64, t82312: f64, t83556: f64, t83559: f64, t83566: f64, t83579: f64, t83582: f64, t83585: f64, t83592: f64, t83596: f64) -> f64 {
    let t83603 = t3231 * t868;
    let t83607 = -9.0_f64 * t22959 * t83556 - t1877 * t6670 * t83559 / 2.0_f64 + 9.0_f64 * t2522 * t6666 * t23792 + 9.0_f64 * t4314 * t1915 * t83566 + t1877 * t82308 * t28 / 2.0_f64 + 9.0_f64 * t4314 * t6666 * t23781 - 3.0_f64 * t1877 * t23290 * t23810 - 9.0_f64 / 2.0_f64 * t22959 * t83579 + 9.0_f64 * t22959 * t83582 - 3.0_f64 * t1877 * t82312 * t83585 + 3.0_f64 * t1877 * t81539 * t23807 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t83592 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t83596 + 9.0_f64 / 2.0_f64 * t2522 * t6666 * t23796 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t83603;
    t83607
}
