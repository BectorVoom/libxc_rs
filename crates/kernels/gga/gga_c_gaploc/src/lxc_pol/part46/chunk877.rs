//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 877/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk877<F: Float>(t1445: F, t2778: F, t574: F, t9127: F, t2876: F, t9453: F, t3159: F, t42085: F, t475: F, t12874: F, t4527: F, t4614: F) -> (F, F, F, F) {
    let t42292 = F::new(0.46011511144704899612e1) * t574 * t1445 * t2778 * t9127;
    let t42296 = t2876 * t9453;
    let t42298 = F::new(0.16683561977530199113e1) * t3159 * t42296;
    let t42299 = t42085 * t475;
    let t42305 = F::new(0.36809208915763919689e2) * t4527 * t4614 * t12874;
    (t42292, t42298, t42299, t42305)
}
