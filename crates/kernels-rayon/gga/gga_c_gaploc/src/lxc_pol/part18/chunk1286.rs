//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1286/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1286(t10012: f64, t10627: f64, t15482: f64, t22633: f64, t11053: f64, t7419: f64, t9805: f64, t1835: f64, t7572: f64, t7573: f64, t10914: f64, t10915: f64, t32897: f64) -> (f64, f64, f64, f64, f64) {
    let t33148 = t10012 * t10627;
    let t33151 = 0.5680433474654925878e0_f64 * t22633 * t15482 * t33148;
    let t33153 = t9805 * t11053 * t7419;
    let t33154 = 0.51762950037793012063e1_f64 * t33153;
    let t33155 = t10627 * t1835;
    let t33158 = 0.69017266717057349418e1_f64 * t7572 * t7573 * t33155;
    let t33164 = 0.42900587942220512002e1_f64 * t10914 * t10915 * t32897;
    (t33151, t33154, t33155, t33158, t33164)
}
