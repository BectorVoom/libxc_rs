//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1543/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1543(t4571: f64, t4644: f64, t1031: f64, t5904: f64, t1022: f64, t1539: f64, t14211: f64, t3071: f64, t1023: f64, t5685: f64, t1616: f64, t4343: f64) -> (f64, f64, f64, f64, f64) {
    let t18008 = t4644 * t4571;
    let t18010 = t5904 * t1031;
    let t18014 = t1539 * t1022;
    let t18015 = t14211 * t18014;
    let t18016 = t3071 * t18015;
    let t18020 = t5685 * t1023;
    let t18021 = t3071 * t18020;
    let t18024 = t1616 * t4343;
    (t18008, t18010, t18016, t18021, t18024)
}
