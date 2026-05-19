//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 923/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk923<F: Float>(t10024: F, t11823: F, t43881: F, t44707: F, t5241: F, t5640: F, t590: F, t11622: F, t2464: F, t2465: F, t825: F, t13641: F, t2013: F) -> (F, F, F, F, F) {
    let t45678 = t11823 * t10024;
    let t45680 = F::cast_from(0.15337170381568299871e1_f64) * t43881;
    let t45684 = F::cast_from(0.13803453343411469884e2_f64) * t5640 * t5241 * t44707 * t590;
    let t45687 = t825 * t2464 * t2465 * t11622;
    let t45688 = F::cast_from(0.42603251059911944084e-1_f64) * t45687;
    let t45689 = t2013 * t13641;
    (t45678, t45680, t45684, t45688, t45689)
}
