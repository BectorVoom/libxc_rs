//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1362/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1362(t652: f64, t6534: f64, t7890: f64, t1458: f64, t7039: f64, t1874: f64, t2035: f64, t4072: f64, t115241: f64, t120986: f64, t120991: f64, t120993: f64, t120995: f64, t120998: f64, t1459: f64, t2314: f64, t26906: f64, t33204: f64, t4034: f64, t6862: f64, t7801: f64, t8450: f64) -> (f64, f64, f64) {
    let t121003 = 2.0_f64 * t652 * t7890 * t6534;
    let t121004 = t7039 * t1458;
    let t121006 = 2.0_f64 * t121004 * t1874;
    let t121007 = t2035 * t4072;
    let t121009 = 2.0_f64 * t121007 * t1874;
    let t121017 = -2.0_f64 * t652 * t6862 * t7801 - 2.0_f64 * t115241 * t1459 - 2.0_f64 * t2314 * t33204 + 3.0_f64 * t26906 * t8450 - 2.0_f64 * t33204 * t4034 - t120986 + t120991 - t120993 - t120995 - t120998 - t121003 - t121006 - t121009;
    (t121004, t121007, t121017)
}
