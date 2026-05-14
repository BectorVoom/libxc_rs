//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1185/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1185<F: Float>(t14759: F, t287: F, t7005: F, t108685: F, t6242: F, t6243: F, t6249: F, t6250: F, t108639: F, t7006: F, t27574: F, t28662: F, t28660: F, t111922: F, t14868: F, t231: F, t25077: F, t2735: F, t28579: F, t28584: F, t54875: F, t6035: F, t6045: F, t6241: F, t6821: F, t7000: F, t704: F, t70457: F, t98462: F, t992: F) -> (F, F) {
    let t111939 = t14759 * t287 * t7005;
    let t111953 = 0.17780800291358024692e0 * t6242 * t108685 * t6243;
    let t111956 = 0.17780800291358024692e0 * t6249 * t108685 * t6250;
    let t111959 = t7006 * t108639;
    let t111967 = t27574 * t28662;
    let t111968 = t28660 * t111967;
    let t111970 = 0.24167761770734866966e0 * t111939 * t6821 - 0.10001700163888888889e0 * t54875 * t6241 * t7000 - 0.20003400327777777778e0 * t28579 * t28584 - 0.10001700163888888889e0 * t6242 * t6045 * t231 * t14868 + t111953 - t111956 - 0.18122740165211489339e1 * t70457 * t111922 - 0.26853068634149852185e-1 * t111959 + 0.33339000546296296298e-1 * t98462 + 0.33339000546296296298e-1 * t25077 * t6035 * t704 * t992 * t2735 + 0.16111841180489911311e0 * t111968;
    (t111967, t111970)
}
