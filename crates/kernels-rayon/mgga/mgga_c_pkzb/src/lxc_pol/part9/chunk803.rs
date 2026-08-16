//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 803/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk803(t5737: f64, t684: f64, t1899: f64, t1971: f64, t1976: f64, t2874: f64, t730: f64, t5519: f64, t5522: f64, t5525: f64, t5539: f64, t228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5738 = t5737 * t684;
    let t5740 = 6.0_f64 * t1899 * t5738;
    let t5742 = t1976 * t1971 * t2874;
    let t5744 = 0.51947577317044391277e2_f64 * t730 * t5742;
    let t5745 = 0.55403703703703703703e-1_f64 * t5519;
    let t5749 = -t5745 + 0.71233333333333333332e-1_f64 * t5522 - 0.53424999999999999999e-1_f64 * t5525 + 0.53425e-1_f64 * t5539;
    let t5751 = 0.621814e-1_f64 * t5749 * t228;
    (t5738, t5740, t5742, t5744, t5745, t5749, t5751)
}
