//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1309/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1309<F: Float>(t10275: F, t1238: F, t10197: F, t10204: F, t10208: F, t10214: F, t11456: F, t22469: F, t22475: F, t22945: F, t22951: F, t23054: F, t2380: F, t2396: F, t2411: F, t26970: F, t26981: F, t27020: F, t28147: F, t300: F, t3185: F, t3202: F, t3206: F, t3880: F, t824: F, t8254: F, t8450: F) -> F {
    let t31755 = t1238 * t10275;
    let t31765 = -F::new(0.85748036236139473944e-3) * t22469 + F::new(0.38586616306262763276e-2) * t2380 * t300 * t2411 * t3880 * t10214 - F::new(0.25724410870841842184e-2) * t3185 * t27020 * t10208 + F::new(0.12862205435420921092e-2) * t3206 * t27020 * t10204 + F::new(0.12862205435420921092e-2) * t3206 * t8254 * t2396 * t3880 * t824 + t22475 + F::new(0.45732285992607719436e-2) * t31755 + F::new(0.64311027177104605458e-3) * t8450 * t23054 * t28147 * t11456 + F::new(0.21722835846488666732e-1) * t10197 * t3202 + F::new(0.85748036236139473944e-3) * t26970 + t22945 + t22951 - F::new(0.17149607247227894789e-2) * t26981;
    t31765
}
