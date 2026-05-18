//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 719/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk719<F: Float>(t1969: F, t20556: F, t446: F, t4714: F, t925: F, t4668: F, t9073: F, t1017: F, t4458: F, t12571: F, t20536: F, t20540: F, t20543: F, t20547: F, t20551: F, t20554: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20557 = t1969 * t20556;
    let t20558 = t446 * t20557;
    let t20560 = t925 * t4714;
    let t20561 = t1969 * t20560;
    let t20562 = t446 * t20561;
    let t20564 = t925 * t4668;
    let t20565 = t9073 * t20564;
    let t20566 = t446 * t20565;
    let t20568 = t4458 * t1017;
    let t20569 = t1969 * t20568;
    let t20570 = t446 * t20569;
    let t20573 = -F::new(5.0) / F::new(81.0) * t20536 - t20540 / F::new(3.0) + t20543 / F::new(3.0) + t20547 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t20551 - t20554 / F::new(9.0) + t20558 / F::new(6.0) + t20562 / F::new(6.0) - t20566 / F::new(3.0) - t20570 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t12571;
    (t20557, t20558, t20560, t20561, t20562, t20564, t20565, t20566, t20568, t20569, t20570, t20573)
}
