//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 705/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk705(t22839: f64, t3766: f64, t556: f64, t598: f64, t213: f64, t1998: f64, t236: f64, t3734: f64, t3872: f64, t6952: f64, t281: f64, t6931: f64) -> (f64, f64, f64, f64, f64) {
    let t22840 = t22839 * t3766;
    let t22842 = t556 * t556;
    let t22843 = 1.0_f64 / t22842;
    let t22844 = t598 * t22843;
    let t22845 = t22844 * t213;
    let t22847 = t1998 * t236 * t3734;
    let t22848 = t22845 * t22847;
    let t22850 = t6952 * t3872;
    let t22852 = t6931 * t281;
    (t22840, t22845, t22848, t22850, t22852)
}
