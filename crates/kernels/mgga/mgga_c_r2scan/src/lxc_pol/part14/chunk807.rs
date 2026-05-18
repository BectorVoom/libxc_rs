//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 807/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk807<F: Float>(t1048: F, t2262: F, t2867: F, t1234: F, t2859: F, t2858: F, t481: F, t795: F, t2266: F, t5027: F, t5029: F, t4703: F, t4721: F, t4880: F, t4891: F, t4901: F, t4964: F, t4967: F, t6943: F, t6946: F, t6947: F, t6948: F, t6949: F, t6950: F, t6951: F, t6952: F, t6954: F) -> (F, F, F, F, F, F) {
    let t7141 = t1048 * t2867 * t2262;
    let t7142 = t2859 * t1234;
    let t7143 = t2858 * t7142;
    let t7144 = F::new(6.0) * t7143;
    let t7145 = t481 * t795;
    let t7147 = t2266 * t2867 * t7145;
    let t7148 = F::new(6.0) * t7147;
    let t7149 = F::new(16.0) * t5027;
    let t7150 = F::new(0.11696447245269292414e1) * t5029;
    let t7151 = t6943 + t4880 - t6946 + t6947 + t6948 - t4891 - t6949 - t6950 + t4703 - t6951 - t6952 + t4901 + t4721 - t4964 + t4967 + t6954;
    (t7141, t7144, t7148, t7149, t7150, t7151)
}
