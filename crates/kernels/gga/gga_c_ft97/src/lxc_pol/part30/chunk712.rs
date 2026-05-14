//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 712/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk712<F: Float>(t230: F, t820: F, t420: F, t7470: F, t33411: F, t7006: F, t291: F, t4092: F, t52: F, t7457: F, t811: F, t19039: F, t19101: F, t19107: F, t19132: F, t28603: F, t28677: F, t28680: F, t31462: F, t33415: F, t33885: F, t33889: F, t33894: F, t5265: F, t7590: F, t812: F, t821: F) -> (F, F, F, F, F, F, F, F) {
    let t33897 = t230 * t820;
    let t33898 = t420 * t33897;
    let t33899 = t7470 * t33898;
    let t33903 = 0.30209702213418583705e-1 * t7006 * t33411;
    let t33906 = t4092 * t291;
    let t33908 = t52 * t7457 * t811;
    let t33912 = t52 * t7457 * t820;
    let t33917 = 0.20527106943485609994e0 * t19039 * t7590 * t812 - 0.10263553471742804997e0 * t5265 * t7590 * t821 - 0.82108427773942439976e0 * t19101 * t33885 + 0.41054213886971219988e0 * t19107 * t33889 - 0.18125821328051150223e0 * t28677 * t33894 + 0.18125821328051150223e0 * t28680 * t33899 - t33903 - 0.30209702213418583705e-1 * t28603 * t33415 + 0.45306850413028723348e0 * t33906 * t33908 - 0.22653425206514361674e0 * t31462 * t33912 + 0.41054213886971219988e0 * t19132 * t33885;
    (t33897, t33898, t33899, t33903, t33906, t33908, t33912, t33917)
}
