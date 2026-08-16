//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1189/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1189(t11727: f64, t52835: f64, t11832: f64, t1706: f64, t11887: f64, t52834: f64, t11913: f64, t11880: f64, t15908: f64, t9467: f64, t9882: f64, t5154: f64, t9919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t53472 = t52835 * t11727;
    let t53490 = t1706 * t11832;
    let t53565 = t52834 * t11887;
    let t53592 = t52834 * t11913;
    let t53613 = t52834 * t11880;
    let t53777 = t15908 * t9467;
    let t53779 = t15908 * t9882;
    let t53798 = t5154 * t9919;
    (t53472, t53490, t53565, t53592, t53613, t53777, t53779, t53798)
}
