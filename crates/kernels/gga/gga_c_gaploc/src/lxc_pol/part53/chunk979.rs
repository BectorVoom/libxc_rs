//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 979/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk979<F: Float>(t47508: F, t9824: F, t41413: F, t41418: F, t41422: F, t41428: F, t13891: F, t2033: F, t549: F, t12256: F, t9972: F, t13866: F, t5782: F) -> (F, F, F, F, F, F, F, F) {
    let t47509 = t47508 * t9824;
    let t47511 = F::new(0.38342925953920749677e0) * t41413;
    let t47512 = F::new(0.38342925953920749677e0) * t41418;
    let t47513 = F::new(0.85206502119823888171e-1) * t41422;
    let t47515 = F::new(0.51123901271894332903e0) * t41428;
    let t47517 = t2033 * t549 * t13891;
    let t47519 = t12256 * t9972;
    let t47527 = t5782 * t13866;
    (t47509, t47511, t47512, t47513, t47515, t47517, t47519, t47527)
}
