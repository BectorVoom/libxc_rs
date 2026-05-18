//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 644/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk644<F: Float>(t237: F, t3611: F, t3587: F, t1116: F, t2860: F, t1954: F, t3591: F, t722: F, t730: F, t3604: F, t713: F, t1976: F) -> (F, F, F, F, F, F, F, F) {
    let t3612 = t237 * t3611;
    let t3614 = F::new(0.19751673498613801407e-1) * t237 * t3587;
    let t3616 = F::new(0.11696447245269292414e1) * t2860 * t1116;
    let t3618 = t1954 * t3591 * t722;
    let t3620 = F::new(0.11696447245269292414e1) * t730 * t3618;
    let t3622 = t713 * t3604 * t722;
    let t3624 = F::new(0.5848223622634646207e0) * t730 * t3622;
    let t3625 = t1976 * t3591;
    (t3612, t3614, t3616, t3618, t3620, t3622, t3624, t3625)
}
