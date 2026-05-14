//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1242/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1242<F: Float>(t1634: F, t164: F, t1733: F, t179: F, t20155: F, t20157: F, t20164: F, t24098: F, t24311: F, t24316: F, t24320: F, t24322: F, t24324: F, t24337: F, t2645: F, t5279: F, t8962: F) -> (F,) {
    let t24345 = -0.42874018118069736972e-3 * t2645 * t179 * t24311 - 0.21437009059034868486e-3 * t2645 * t179 * t24316 - 0.80031500487063509015e-2 * t24320 + 0.20007875121765877254e-2 * t24322 + 0.85748036236139473944e-3 * t1733 * t179 * t24324 * t164 + 0.7558530601555998074e-1 * t20155 + 7.0 / 6.0 * t20157 - 0.90702367218671976884e-1 * t20164 - 0.42874018118069736972e-2 * t5279 * t179 * t24098 * t164 - 0.21437009059034868486e-3 * t2645 * t179 * t24337 - 0.42874018118069736972e-2 * t5279 * t179 * t8962 * t1634;
    (t24345,)
}
