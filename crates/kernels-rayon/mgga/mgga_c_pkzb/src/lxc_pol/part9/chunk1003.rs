//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1003/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1003(t8054: f64, t8066: f64, t871: f64, t1201: f64, t2295: f64, t7930: f64, t6090: f64, t6093: f64, t6180: f64, t6183: f64, t6211: f64, t7947: f64, t7950: f64, t7955: f64, t7959: f64, t7961: f64, t7967: f64) -> (f64, f64, f64, f64) {
    let t8067 = t8054 + t8066;
    let t8068 = t8067 * t871;
    let t8071 = t1201 * t2295;
    let t8076 = 0.60385e0_f64 * t7930;
    let t8085 = -t6211 + 0.80513333333333333334e0_f64 * t6090 - 0.301925e0_f64 * t6093 - t8076 + 0.905775e0_f64 * t7947 + 0.27595e0_f64 * t7950 + 0.258925e1_f64 * t7959 + 0.16504875e0_f64 * t7961 - 0.16557e0_f64 * t6180 - 0.16557e0_f64 * t6183 + 0.40256666666666666667e0_f64 * t7955 - 0.258925e1_f64 * t7967;
    (t8067, t8068, t8071, t8085)
}
