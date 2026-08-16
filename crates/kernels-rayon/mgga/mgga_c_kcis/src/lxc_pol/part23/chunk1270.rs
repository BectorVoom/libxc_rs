//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1270/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1270(t1598: f64, t51799: f64, t52852: f64, t1014: f64, t28412: f64, t1982: f64, t4121: f64, t303: f64, t4125: f64, t10470: f64, t15802: f64, t4158: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98733 = t51799 * t1598;
    let t98736 = t52852 * t1598;
    let t98743 = t1014 * t28412;
    let t98744 = 0.22109259259259259258e-2_f64 * t98743;
    let t98745 = t1982 * t4121;
    let t98747 = t303 * t98745 * t4125;
    let t98751 = t10470 * t4158 * t552 * t15802;
    (t98733, t98736, t98743, t98744, t98747, t98751)
}
