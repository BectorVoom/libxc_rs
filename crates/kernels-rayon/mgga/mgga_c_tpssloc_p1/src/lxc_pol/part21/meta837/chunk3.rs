//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2981/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2981(t10422: f64, t17648: f64, t3070: f64, t10214: f64, t1031: f64, t17701: f64, t17877: f64, t18036: f64, t2979: f64, t378: f64, t42508: f64, t42541: f64, t49799: f64, t49801: f64, t49808: f64, t49810: f64, t49818: f64, t49820: f64, t59668: f64, t59672: f64, t59696: f64, t59725: f64, t59742: f64, t973: f64, t977: f64) -> f64 {
    let t62234 = t3070 * t10422 * t17648;
    let t62258 = t49799 / 3456.0_f64 + 5.0_f64 / 5184.0_f64 * t49801 + t42541 * t18036 / 1152.0_f64 + t42508 * t17701 / 432.0_f64 - t62234 / 1728.0_f64 - t973 * t977 * t59696 / 144.0_f64 - t973 * t2979 * t59742 / 36.0_f64 + t973 * t2979 * t59668 / 108.0_f64 + t973 * t2979 * t59672 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t973 * t10214 * t59725 - t49808 / 3456.0_f64 - t17877 * t1031 * t378 / 288.0_f64 + t49810 / 3456.0_f64 - t49818 / 3456.0_f64 + t49820 / 2304.0_f64;
    t62258
}
