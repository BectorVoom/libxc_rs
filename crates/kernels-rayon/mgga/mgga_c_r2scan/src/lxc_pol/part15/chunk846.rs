//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 846/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk846(t2148: f64, t7629: f64, t7628: f64, t1398: f64, t5: f64, t966: f64, t2804: f64, t378: f64, t5202: f64, t5205: f64, t5209: f64, t5212: f64, t5213: f64, t5218: f64, t5220: f64, t5225: f64, t5230: f64, t5233: f64, t5235: f64) -> (f64, f64, f64, f64) {
    let t7630 = t2148 * t7629;
    let t7632 = 0.23287303101564395622e-1_f64 * t7628 * t7630;
    let t7637 = t5 * t1398 * t966;
    let t7641 = 10.0_f64 / 3.0_f64 * t5 * t378 * t2804;
    let t7645 = -t5202 - t5205 - t5209 + t5212 + 0.53360572013155555553e-2_f64 * t5213 - t5218 - 0.67745118933333333332e-2_f64 * t5220 - t5225 + t5230 - t5233 - 0.54217906501508699211e-2_f64 * t5235;
    (t7632, t7637, t7641, t7645)
}
