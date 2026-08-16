//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1133/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1133(t1181: f64, t6074: f64, t7351: f64, t7564: f64, t30887: f64, t30890: f64, t30893: f64, t30905: f64, t30908: f64, t35040: f64, t35042: f64, t35052: f64, t37363: f64, t39653: f64, t39658: f64, t39661: f64, t39665: f64, t39669: f64, t39673: f64, t39675: f64, t39679: f64) -> f64 {
    let t39683 = t7564 * t1181 * t7351 * t6074;
    let t39685 = 0.68598428988911579156e-2_f64 * t39653 - t30887 - t30890 + 0.14291339372689912324e-3_f64 * t30893 - t30905 - t30908 + 0.7640625e-2_f64 * t39658 + t39661 / 24.0_f64 + t35040 + t35042 - 0.32155513588552302729e-2_f64 * t39665 + 0.53592522647587171215e-3_f64 * t39669 - t37363 - 0.7145669686344956162e-4_f64 * t39673 - t35052 + 0.15724046144802076034e-3_f64 * t39675 + 0.15724046144802076034e-3_f64 * t39679 + 0.94344276868812456204e-3_f64 * t39683;
    t39685
}
