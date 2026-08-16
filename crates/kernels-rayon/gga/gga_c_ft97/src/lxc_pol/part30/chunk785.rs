//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 785/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk785(t52: f64, t7457: f64, t820: f64, t19039: f64, t19101: f64, t19107: f64, t19132: f64, t28603: f64, t28677: f64, t28680: f64, t31462: f64, t33415: f64, t33885: f64, t33889: f64, t33894: f64, t33899: f64, t33903: f64, t33906: f64, t33908: f64, t5265: f64, t7590: f64, t812: f64, t821: f64) -> (f64, f64) {
    let t33912 = t52 * t7457 * t820;
    let t33917 = 0.20527106943485609994e0_f64 * t19039 * t7590 * t812 - 0.10263553471742804997e0_f64 * t5265 * t7590 * t821 - 0.82108427773942439976e0_f64 * t19101 * t33885 + 0.41054213886971219988e0_f64 * t19107 * t33889 - 0.18125821328051150223e0_f64 * t28677 * t33894 + 0.18125821328051150223e0_f64 * t28680 * t33899 - t33903 - 0.30209702213418583705e-1_f64 * t28603 * t33415 + 0.45306850413028723348e0_f64 * t33906 * t33908 - 0.22653425206514361674e0_f64 * t31462 * t33912 + 0.41054213886971219988e0_f64 * t19132 * t33885;
    (t33912, t33917)
}
