//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1010/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1010<F: Float>(t1465: F, t540: F, t3728: F, t3956: F, t4131: F, t1392: F, t1457: F, t1017: F, t86: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t11823 = t1465 * t1465;
    let t11824 = F::new(1.0) / t11823;
    let t11825 = t540 * t11824;
    let t11826 = t11825 * sigma2;
    let t11832 = t3728 * t3956;
    let t11838 = t3728 * t4131;
    let t11860 = t1392 * t1457;
    let t11862 = t86 * t1017 * t11860;
    (t11824, t11825, t11826, t11832, t11838, t11862)
}
