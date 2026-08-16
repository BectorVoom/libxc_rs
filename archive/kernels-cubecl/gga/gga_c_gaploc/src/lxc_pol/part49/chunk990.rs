//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 990/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk990<F: Float>(t43007: F, t7290: F, t1841: F, t7289: F, t2508: F, t3255: F, t8637: F, t2936: F, t9689: F, t13206: F, t7137: F, t3487: F, t734: F, t9636: F) -> (F, F, F, F, F, F) {
    let t43008 = t7290 * t43007;
    let t43010 = t1841 * t7289 * t43008;
    let t43014 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t8637 * t3255;
    let t43017 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t2936 * t9689;
    let t43019 = F::cast_from(0.20508069947045931423e-1_f64) * t7137 * t13206;
    let t43023 = F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t9636 * t3487 * t734;
    (t43008, t43010, t43014, t43017, t43019, t43023)
}
