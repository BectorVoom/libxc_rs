//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 738/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk738(t5954: f64, t637: f64, t1763: f64, t1942: f64, t1762: f64, t1835: f64, t377: f64, t1946: f64, t1767: f64, t1987: f64, t424: f64, t625: f64) -> (f64, f64, f64, f64, f64) {
    let t5955 = t5954 * t637;
    let t5957 = t1763 * t1942;
    let t5959 = 0.32530743900905219526e-1_f64 * t1762 * t5957;
    let t5960 = t377 * t1835;
    let t5961 = t5960 * t1946;
    let t5963 = 0.28895839882605942646e1_f64 * t1762 * t5961;
    let t5964 = t1767 * t1987;
    let t5966 = 0.96319466275353142157e0_f64 * t1762 * t5964;
    let t5967 = t424 * t625;
    (t5955, t5959, t5963, t5966, t5967)
}
