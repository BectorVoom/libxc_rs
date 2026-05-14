//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 870/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk870<F: Float>(t1022: F, t9636: F, t2009: F, t2021: F, t13150: F, t2013: F, t10007: F, t2925: F, t825: F, t9438: F, t3039: F, t5774: F, t3277: F, t13009: F, t5782: F, t1457: F, t43240: F, t6060: F) -> (F, F, F, F, F, F) {
    let t44080 = t9636 * t1022;
    let t44083 = 0.35750489951850426669e0 * t2021 * t44080 * t2009;
    let t44084 = t2013 * t13150;
    let t44085 = 0.15976219147466979032e-1 * t44084;
    let t44088 = t825 * t9438 * t10007 * t2925;
    let t44089 = 0.15976219147466979032e-1 * t44088;
    let t44090 = t3039 * t5774;
    let t44092 = 0.16683561977530199113e1 * t3277 * t44090;
    let t44093 = t5782 * t13009;
    let t44097 = 0.21450293971110256001e1 * t6060 * t1457 * t43240;
    (t44083, t44085, t44089, t44092, t44093, t44097)
}
