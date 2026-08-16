//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 726/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk726(t1685: f64, t591: f64, t1684: f64, t5946: f64, t1763: f64, t1942: f64, t1762: f64, t1835: f64, t377: f64, t1946: f64, t1767: f64, t1987: f64) -> (f64, f64, f64, f64) {
    let t5947 = t1685 * t591;
    let t5948 = t1684 * t5947;
    let t5950 = 0.254044196e-2_f64 * t5946 * t5948;
    let t5957 = t1763 * t1942;
    let t5959 = 0.32530743900905219526e-1_f64 * t1762 * t5957;
    let t5960 = t377 * t1835;
    let t5961 = t5960 * t1946;
    let t5963 = 0.28895839882605942646e1_f64 * t1762 * t5961;
    let t5964 = t1767 * t1987;
    (t5950, t5959, t5963, t5964)
}
