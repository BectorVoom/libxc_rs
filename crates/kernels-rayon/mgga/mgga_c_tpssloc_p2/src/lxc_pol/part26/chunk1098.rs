//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1098/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1098(t22833: f64, t3809: f64, t2002: f64, t3773: f64, t559: f64, t1878: f64, t557: f64, t3766: f64, t556: f64, t598: f64, t213: f64, t1998: f64, t236: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22834 = t22833 * t3809;
    let t22836 = t3773 * t2002;
    let t22837 = t22836 * t559;
    let t22839 = t1878 * t557;
    let t22840 = t22839 * t3766;
    let t22842 = t556 * t556;
    let t22843 = 1.0_f64 / t22842;
    let t22844 = t598 * t22843;
    let t22845 = t22844 * t213;
    let t22847 = t1998 * t236 * t3734;
    (t22834, t22836, t22837, t22839, t22840, t22842, t22843, t22844, t22845, t22847)
}
