//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 983/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk983<F: Float>(t12311: F, t2554: F, t7064: F, t123: F, t1841: F, t47182: F, t734: F, t1843: F, t47188: F, t47178: F, t9647: F, t39040: F, t5539: F) -> (F, F, F, F, F) {
    let t47597 = t7064 * t12311 * t2554;
    let t47602 = F::new(0.85450291446024714263e-3) * t1841 * t47182 * t123 * t734;
    let t47605 = F::new(0.85450291446024714263e-3) * t1841 * t1843 * t47188;
    let t47607 = t9647 * t1843 * t47178;
    let t47610 = t9647 * t5539 * t39040;
    (t47597, t47602, t47605, t47607, t47610)
}
