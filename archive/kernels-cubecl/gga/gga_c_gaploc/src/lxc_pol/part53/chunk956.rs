//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 956/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk956<F: Float>(t13756: F, t380: F, t12035: F, t6556: F, t39340: F, t921: F, t12032: F, t2497: F, t12148: F, t1382: F, t13838: F, t5559: F, t841: F) -> (F, F, F, F, F, F) {
    let t47054 = F::cast_from(0.37940008847568199465e-1_f64) * t380 * t13756;
    let t47064 = t6556 * t12035;
    let t47071 = t39340 * t921;
    let t47075 = t12032 * t2497;
    let t47077 = t1382 * t12148 * t921;
    let t47080 = t5559 * t13838 * t841;
    (t47054, t47064, t47071, t47075, t47077, t47080)
}
