//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 893/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk893<F: Float>(t13657: F, t4614: F, t833: F, t11784: F, t2617: F, t7810: F, t44713: F, t4820: F, t7513: F, t11780: F, t23000: F, t2679: F, t9805: F) -> (F, F, F, F, F) {
    let t45226 = F::cast_from(0.58281247449959539508e2_f64) * t833 * t4614 * t13657;
    let t45228 = t7810 * t11784 * t2617;
    let t45229 = F::cast_from(0.19171462976960374838e0_f64) * t45228;
    let t45232 = F::cast_from(0.79445533226334281487e-1_f64) * t7513 * t4820 * t44713;
    let t45234 = t23000 * t11780 * t2617;
    let t45238 = t9805 * t11780 * t2679;
    (t45226, t45229, t45232, t45234, t45238)
}
