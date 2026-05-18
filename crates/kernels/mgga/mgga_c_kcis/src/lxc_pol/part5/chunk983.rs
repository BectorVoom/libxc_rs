//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 983/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk983<F: Float>(t1416: F, t3820: F, t11407: F, t2820: F, t3751: F, t86: F, t1465: F, t540: F, t1494: F, t3754: F, t1392: F, t1457: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t11730 = t3820 * t1416;
    let t11746 = F::new(0.12841111111111111111e-1) * t11407;
    let t11814 = t86 * t2820 * t3751;
    let t11823 = t1465 * t1465;
    let t11824 = F::new(1.0) / t11823;
    let t11825 = t540 * t11824;
    let t11826 = t11825 * sigma2;
    let t11854 = t1494 * t3754;
    let t11860 = t1392 * t1457;
    (t11730, t11746, t11814, t11824, t11825, t11826, t11854, t11860)
}
