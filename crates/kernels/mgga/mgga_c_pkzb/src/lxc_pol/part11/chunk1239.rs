//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1239/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1239<F: Float>(t10829: F, t1979: F, t10830: F, t10834: F, t17638: F, t1955: F, t1977: F, t21212: F, t2848: F, t30236: F, t30238: F, t30242: F, t30245: F, t30248: F, t30252: F, t3608: F, t5838: F, t5845: F, t721: F, t7315: F, t7494: F, t9401: F, t9402: F, t9446: F, t9452: F, t9455: F) -> F {
    let t30459 = t10829 * t1979;
    let t30466 = -F::new(0.35089341735807877242e1) * t7494 * t9446 + F::new(0.51947577317044391276e2) * t7315 * t9452 + F::new(0.10389515463408878255e3) * t7315 * t9455 + F::new(0.30762056574649219972e4) * t21212 * t9402 - F::new(0.31168546390226634765e3) * t5838 * t3608 * t2848 - F::new(0.12304822629859687989e5) * t17638 * t10834 * t721 - F::new(0.11696447245269292414e1) * t1955 * t10830 * t721 + F::new(0.17315859105681463759e2) * t1977 * t30459 * t721 + F::new(0.30762056574649219974e4) * t5845 * t9401 * t2848 - t30236 - t30238 + t30242 + t30245 + t30248 - t30252;
    t30466
}
