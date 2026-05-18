//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1310/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1310<F: Float>(t11347: F, t2099: F, t6459: F, t10241: F, t8368: F, t10271: F, t1238: F, t2411: F, t300: F, t3874: F, t10047: F, t10067: F, t22989: F, t23008: F, t2371: F, t26986: F, t26995: F, t27007: F, t27014: F, t27028: F, t27031: F, t3061: F, t3185: F) -> (F, F) {
    let t31771 = t6459 * t2099 * t11347;
    let t31773 = t8368 * t10241;
    let t31777 = t1238 * t10271;
    let t31782 = t300 * t2411 * t3874;
    let t31787 = -t22989 - t26986 / F::new(9.0) + t26995 / F::new(48.0) - F::new(0.85748036236139473944e-3) * t27007 - t23008 + F::new(0.42874018118069736972e-3) * t27014 + F::new(0.14291339372689912324e-3) * t31771 - F::new(0.45732285992607719436e-2) * t31773 - F::new(0.68598428988911579154e-2) * t10047 * t10067 - F::new(0.13719685797782315831e-1) * t31777 - t27028 / F::new(16.0) + t27031 / F::new(24.0) + F::new(0.77173232612525526549e-2) * t3185 * t31782 * t2371 * t3061;
    (t31782, t31787)
}
