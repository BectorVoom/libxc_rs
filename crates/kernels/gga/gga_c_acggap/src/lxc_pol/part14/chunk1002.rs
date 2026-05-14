//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1002/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1002<F: Float>(t7839: F, t9641: F, t1165: F, t2068: F, t604: F, t6069: F, t1181: F, t6074: F, t7351: F, t7564: F, t30887: F, t30890: F, t30893: F, t30905: F, t30908: F, t35040: F, t35042: F, t35052: F, t37363: F, t39653: F, t39658: F, t39661: F, t39665: F, t39669: F, t39673: F) -> (F,) {
    let t39675 = t7839 * t9641;
    let t39679 = t2068 * t1165 * t604 * t6069;
    let t39683 = t7564 * t1181 * t7351 * t6074;
    let t39685 = 0.68598428988911579156e-2 * t39653 - t30887 - t30890 + 0.14291339372689912324e-3 * t30893 - t30905 - t30908 + 0.7640625e-2 * t39658 + t39661 / 24.0 + t35040 + t35042 - 0.32155513588552302729e-2 * t39665 + 0.53592522647587171215e-3 * t39669 - t37363 - 0.7145669686344956162e-4 * t39673 - t35052 + 0.15724046144802076034e-3 * t39675 + 0.15724046144802076034e-3 * t39679 + 0.94344276868812456204e-3 * t39683;
    (t39685,)
}
