//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1241/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1241(t792: f64, t9573: f64, t3275: f64, t3276: f64, t11550: f64, t983: f64, t3262: f64, t43764: f64, t43766: f64, t43770: f64, t43774: f64, t43778: f64, t43780: f64, t43782: f64, t43783: f64, t43785: f64, t43787: f64, t43789: f64, t43791: f64, t43795: f64, t43797: f64) -> (f64, f64, f64) {
    let t43798 = t9573 * t792;
    let t43801 = 5.0_f64 / 8.0_f64 * t3275 * t3276 * t43798;
    let t43802 = t11550 * t983;
    let t43805 = 15.0_f64 / 8.0_f64 * t3262 * t3276 * t43802;
    let t43806 = -t43764 + t43766 + t43770 + t43774 - t43778 - t43780 + t43782 + t43783 - t43785 - t43787 - t43789 + t43791 + t43795 + t43797 - t43801 - t43805;
    (t43801, t43805, t43806)
}
