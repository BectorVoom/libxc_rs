//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1159/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1159(t3262: f64, t3276: f64, t42855: f64, t11506: f64, t40421: f64, t11523: f64, t11545: f64, t35373: f64, t481: f64, t39263: f64, t4176: f64, t2449: f64, t3696: f64, t42818: f64, t42822: f64, t42824: f64, t42826: f64, t42832: f64, t42836: f64, t42840: f64, t42843: f64, t42845: f64, t42850: f64, t42854: f64) -> (f64, f64, f64, f64, f64) {
    let t42858 = 15.0_f64 / 16.0_f64 * t3262 * t3276 * t42855;
    let t42860 = 3.0_f64 / 2.0_f64 * t11506 * t40421;
    let t42862 = 5.0_f64 / 8.0_f64 * t11523 * t11545;
    let t42863 = t35373 * t481;
    let t42866 = 3.0_f64 * t39263 * t4176 * t42863;
    let t42867 = 2.0_f64 * t2449 * t3696 + t42818 + t42822 + t42824 - t42826 + t42832 + t42836 + t42840 - t42843 - t42845 - t42850 + t42854 - t42858 + t42860 - t42862 - t42866;
    (t42858, t42860, t42862, t42866, t42867)
}
