//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 843/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk843(t8919: f64, t8944: f64, t8993: f64, t9017: f64, t158: f64, t3466: f64, t5418: f64, t633: f64, t2678: f64, t2702: f64, t1790: f64, t3487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9019 = t8919 + t8944 + t8993 + t9017;
    let t9020 = t9019 * t158;
    let t9033 = t5418 * t3466;
    let t9034 = t9033 * t633;
    let t9037 = t2678 * t2702;
    let t9042 = t1790 * t3487;
    (t9019, t9020, t9033, t9034, t9037, t9042)
}
