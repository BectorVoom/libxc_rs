//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 704/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk704<F: Float>(t3040: F, t3267: F, t10012: F, t1022: F, t9438: F, t2684: F, t10007: F, t825: F, t313: F, t9014: F, t1645: F, t3251: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13140 = F::new(0.35750489951850426669e0) * t3267 * t3040;
    let t13141 = t10012 * t1022;
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    let t13144 = F::new(0.15976219147466979032e-1) * t13143;
    let t13149 = t10007 * t1022;
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13152 = F::new(0.15976219147466979032e-1) * t13151;
    let t13153 = t313 * t9014;
    let t13154 = t1645 * t3251;
    (t13140, t13141, t13142, t13144, t13149, t13150, t13152, t13153, t13154)
}
