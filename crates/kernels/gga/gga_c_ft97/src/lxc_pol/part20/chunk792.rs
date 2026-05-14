//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 792/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk792<F: Float>(t24816: F, t9803: F, t242: F, t24405: F, t24430: F, t24395: F, t265: F, t729: F, t24413: F, t1456: F, t2459: F, t6194: F, t713: F, t1882: F, t6172: F, t6189: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24817 = t9803 * t24816;
    let t24820 = t242 * t24405;
    let t24823 = t242 * t24430;
    let t24827 = t729 * t265 * t24395;
    let t24830 = t242 * t24413;
    let t24834 = t729 * t1456 * t2459;
    let t24838 = t729 * t6194 * t713;
    let t24841 = t1882 * t6172;
    let t24843 = t1882 * t6189;
    (t24817, t24820, t24823, t24827, t24830, t24834, t24838, t24841, t24843)
}
