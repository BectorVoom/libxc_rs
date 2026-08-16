//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 728/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk728(t1966: f64, t2009: f64, t5589: f64, t5748: f64, t5754: f64, t5755: f64, t5761: f64, t5766: f64, t5770: f64, t5774: f64, t5777: f64, t5781: f64, t5782: f64, t5785: f64, t5787: f64, t5793: f64, t5794: f64, t664: f64, t674: f64, t682: f64, t687: f64, t705: f64) -> f64 {
    let t5797 = -0.24828486201251232145e5_f64 * t5748 * t2009 * t5589 - t5754 + 0.96491876992155210402e2_f64 * t687 * t5755 * t664 + t5761 + t5766 + t5770 - t5774 - t5777 - 6.0_f64 * t674 * t682 * t1966 - 0.57895126195293126243e3_f64 * t5781 * t5782 + 0.6207121550312808036e4_f64 * t5785 * t5787 - t5793 - 0.11696447245269292414e1_f64 * t705 * t5794;
    t5797
}
