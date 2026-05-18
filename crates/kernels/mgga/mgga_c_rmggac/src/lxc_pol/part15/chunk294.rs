//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 294/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk294<F: Float>(t537: F, t809: F, t278: F, t815: F, t90: F, t541: F, t95: F, t547: F, t820: F, t101: F, t102: F, t544: F, t548: F, t832: F, t87: F, t98: F, rho1: F, tau1: F) -> (F, F, F, F, F, F, F, F) {
    let t1710 = t537 * t537;
    let t1711 = t809 * t1710;
    let t1714 = t278 + t815;
    let t1715 = t90 * t1714;
    let t1718 = t541 * rho1;
    let t1720 = F::new(1.0) / t95 / t1718;
    let t1721 = tau1 * t1720;
    let t1726 = t547 * t547;
    let t1727 = t820 * t1726;
    let t1730 = -t1714;
    let t1731 = t101 * t1730;
    let t1734 = F::new(20.0) / F::new(9.0) * t87 * t1711 + F::new(10.0) / F::new(3.0) * t87 * t1715 + F::new(80.0) / F::new(9.0) * t1721 * t102 - F::new(100.0) / F::new(9.0) * t544 * t548 + F::new(20.0) / F::new(9.0) * t98 * t1727 + F::new(10.0) / F::new(3.0) * t98 * t1731 - t832;
    (t1710, t1711, t1714, t1715, t1721, t1726, t1730, t1734)
}
