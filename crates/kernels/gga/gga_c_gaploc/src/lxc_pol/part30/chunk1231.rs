//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1231/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1231<F: Float>(t2508: F, t32356: F, t688: F, t779: F, t10682: F, t2060: F, t1897: F, t27348: F, t954: F, t23433: F, t2936: F, t10697: F, t29020: F, t9647: F) -> (F, F, F, F, F) {
    let t32471 = F::new(0.15381052460284448567e-1) * t2508 * t779 * t32356 * t688;
    let t32474 = F::new(0.76905262301422242837e-2) * t2508 * t2060 * t10682;
    let t32477 = F::new(0.76905262301422242837e-2) * t1897 * t954 * t27348;
    let t32480 = F::new(0.23071578690426672851e-1) * t1897 * t2936 * t23433;
    let t32482 = t9647 * t10697 * t29020;
    (t32471, t32474, t32477, t32480, t32482)
}
