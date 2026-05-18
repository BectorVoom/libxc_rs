//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 937/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk937<F: Float>(t2009: F, t2021: F, t44080: F, t13150: F, t2013: F, t10007: F, t2925: F, t825: F, t9438: F, t3039: F, t5774: F, t3277: F) -> (F, F, F, F) {
    let t44083 = F::new(0.35750489951850426669e0) * t2021 * t44080 * t2009;
    let t44084 = t2013 * t13150;
    let t44085 = F::new(0.15976219147466979032e-1) * t44084;
    let t44088 = t825 * t9438 * t10007 * t2925;
    let t44089 = F::new(0.15976219147466979032e-1) * t44088;
    let t44090 = t3039 * t5774;
    let t44092 = F::new(0.16683561977530199113e1) * t3277 * t44090;
    (t44083, t44085, t44089, t44092)
}
