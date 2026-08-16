//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1227/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1227(t1795: f64, t2061: f64, t10456: f64, t1165: f64, t13146: f64, t1799: f64, t18627: f64, t18680: f64, t18898: f64, t2056: f64, t2105: f64, t4347: f64, t5801: f64, t5815: f64, t645: f64, t7798: f64) -> (f64, f64) {
    let t18903 = t1795 * t2061;
    let t18919 = 4.0_f64 * t10456 * t1799 + 2.0_f64 * t1165 * t18627 + 2.0_f64 * t13146 * t1799 + 2.0_f64 * t1799 * t7798 + 4.0_f64 * t18898 * t645 + 4.0_f64 * t2056 * t5815 + 2.0_f64 * t2105 * t5801 + 4.0_f64 * t4347 * t5815 + t18680 + 2.0_f64 * t18903;
    (t18903, t18919)
}
