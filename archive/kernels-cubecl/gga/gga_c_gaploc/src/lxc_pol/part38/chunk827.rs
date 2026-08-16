//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 827/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk827<F: Float>(t1358: F, t161: F, t37975: F, t44255: F, t11280: F, t20883: F, t6525: F, t42539: F, t42546: F, t10166: F, t10252: F, t9074: F) -> (F, F, F, F, F) {
    let t44258 = F::cast_from(0.37940008847568199464e-1_f64) * t1358 * t37975 * t161 * t44255;
    let t44261 = t6525 * t11280 * t20883;
    let t44262 = F::cast_from(0.35568758294595186999e-2_f64) * t44261;
    let t44263 = F::cast_from(0.47425011059460249332e-2_f64) * t42539;
    let t44264 = F::cast_from(0.94850022118920498664e-2_f64) * t42546;
    let t44266 = t9074 * t10166 * t10252;
    (t44258, t44262, t44263, t44264, t44266)
}
