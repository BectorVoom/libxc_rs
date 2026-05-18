//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 891/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk891<F: Float>(t309: F, t44600: F, t294: F, t9577: F, t9570: F, t342: F, t784: F, t8639: F, t43537: F, t3051: F, t963: F, t926: F) -> (F, F, F, F, F, F, F) {
    let t44601 = t309 * t44600;
    let t44674 = t294 * t9577;
    let t44700 = t294 * t9570;
    let t44716 = F::new(5.0) / F::new(54.0) * t342 * t8639 * t784;
    let t44776 = F::new(140.0) / F::new(243.0) * t43537;
    let t44950 = t3051 * t963;
    let t45304 = t3051 * t926;
    (t44601, t44674, t44700, t44716, t44776, t44950, t45304)
}
