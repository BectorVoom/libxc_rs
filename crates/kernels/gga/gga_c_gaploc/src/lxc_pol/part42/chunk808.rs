//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 808/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk808<F: Float>(t25359: F, t2615: F, t9438: F, t2344: F, t550: F, t1358: F, t161: F, t37975: F, t11280: F, t20883: F, t6525: F, t42539: F) -> (F, F, F, F, F) {
    let t44133 = t2615 * t9438 * t25359;
    let t44255 = t550 * t2344;
    let t44258 = F::new(0.37940008847568199464e-1) * t1358 * t37975 * t161 * t44255;
    let t44261 = t6525 * t11280 * t20883;
    let t44262 = F::new(0.35568758294595186999e-2) * t44261;
    let t44263 = F::new(0.47425011059460249332e-2) * t42539;
    (t44133, t44255, t44258, t44262, t44263)
}
