//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1108/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1108(t21499: f64, t1066: f64, t154: f64, t18060: f64, t276: f64, t735: f64, t7620: f64, t17867: f64, t2104: f64, t2911: f64, t2064: f64, t2922: f64, t2924: f64) -> (f64, f64, f64, f64, f64) {
    let t21500 = 0.28582678745379824648e-3_f64 * t21499;
    let t21538 = t276 * t154 * t18060 * t1066;
    let t21542 = t735 * t7620;
    let t21543 = t21542 / 54.0_f64;
    let t21623 = t2104 * t17867 * t2911;
    let t21624 = 0.28582678745379824648e-3_f64 * t21623;
    let t21626 = t2922 * t2064 * t2924;
    (t21500, t21538, t21543, t21624, t21626)
}
