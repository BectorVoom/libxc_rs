//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 506/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk506<F: Float>(t2496: F, t760: F, t128: F, t131: F, t136: F, t2457: F, t2470: F, t684: F, t692: F, t2435: F, t2439: F) -> (F, F, F, F, F, F, F, F) {
    let t2498 = F::cast_from(0.17315859105681463759e2_f64) * t760 * t2496;
    let t2501 = F::new(1.0) / t131 / t128 * t136;
    let t2502 = t2501 * t2457;
    let t2504 = t684 * t2470;
    let t2507 = F::new(1.0)/F::sqrt(t128);
    let t2508 = t2507 * t136;
    let t2509 = t2508 * t2457;
    let t2511 = t692 * t2470;
    let t2514 = -F::cast_from(0.57538888888888888889e0_f64) * t2502 + F::cast_from(0.11507777777777777778e1_f64) * t2504 + F::cast_from(0.40256666666666666667e0_f64) * t2435 + F::new(0.366775e-1) * t2509 + F::new(0.73355e-1) * t2511 + F::new(0.137975e0) * t2439;
    (t2498, t2501, t2502, t2504, t2508, t2509, t2511, t2514)
}
