//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 785/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk785<F: Float>(t52: F, t7457: F, t820: F, t19039: F, t19101: F, t19107: F, t19132: F, t28603: F, t28677: F, t28680: F, t31462: F, t33415: F, t33885: F, t33889: F, t33894: F, t33899: F, t33903: F, t33906: F, t33908: F, t5265: F, t7590: F, t812: F, t821: F) -> (F, F) {
    let t33912 = t52 * t7457 * t820;
    let t33917 = F::new(0.20527106943485609994e0) * t19039 * t7590 * t812 - F::new(0.10263553471742804997e0) * t5265 * t7590 * t821 - F::new(0.82108427773942439976e0) * t19101 * t33885 + F::new(0.41054213886971219988e0) * t19107 * t33889 - F::new(0.18125821328051150223e0) * t28677 * t33894 + F::new(0.18125821328051150223e0) * t28680 * t33899 - t33903 - F::new(0.30209702213418583705e-1) * t28603 * t33415 + F::new(0.45306850413028723348e0) * t33906 * t33908 - F::new(0.22653425206514361674e0) * t31462 * t33912 + F::new(0.41054213886971219988e0) * t19132 * t33885;
    (t33912, t33917)
}
