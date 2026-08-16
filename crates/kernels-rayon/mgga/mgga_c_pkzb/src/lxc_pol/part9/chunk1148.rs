//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1148/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1148(t6866: f64, t6892: f64, t1721: f64, t600: f64, t7084: f64, t1719: f64, t2639: f64, t164: f64, t5257: f64, t6877: f64, t6904: f64, t2575: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19958 = t6892 * t6866;
    let t19961 = t7084 * t1721 * t600;
    let t19965 = t2639 * t1719;
    let t19966 = t19965 * t164;
    let t19970 = t5257 * t6877;
    let t19972 = t6892 * t6904;
    let t19974 = t2575 * t1719;
    (t19958, t19961, t19965, t19966, t19970, t19972, t19974)
}
