//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 589/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk589<F: Float>(t3281: F, t5676: F, t2530: F, t2610: F, t2365: F, t2033: F, t1445: F, t9596: F, t1457: F, t3209: F, t325: F, t701: F) -> (F, F, F, F, F, F) {
    let t9942 = F::new(0.29792074959875355558e-1) * t5676 * t3281;
    let t9943 = t2610 * t2530;
    let t9944 = t2365 * t9943;
    let t9946 = F::new(0.29792074959875355558e-1) * t2033 * t9944;
    let t9947 = t1445 * t9596;
    let t9950 = t1457 * t9596;
    let t9953 = t325 * t3209;
    let t9954 = t9953 * t701;
    (t9942, t9946, t9947, t9950, t9953, t9954)
}
