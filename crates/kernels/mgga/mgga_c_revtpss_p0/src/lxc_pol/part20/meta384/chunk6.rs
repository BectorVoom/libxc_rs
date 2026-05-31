//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1408/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1408<F: Float>(t11064: F, t11075: F, t1940: F, t2394: F, t2408: F, t2832: F, t39760: F, t39764: F, t39767: F, t39770: F, t39773: F, t39775: F, t39778: F, t39780: F, t39783: F, t39786: F, t39791: F, t39795: F, t4541: F) -> F {
    let t41150 = F::cast_from(12.0_f64) * t11064 * t1940 * t2408 * t2832 + F::cast_from(36.0_f64) * t11075 * t2394 * t4541 + t39760 - t39764 + t39767 + t39770 + t39773 - t39775 + t39778 + t39780 - t39783 - t39786 - t39791 - t39795;
    t41150
}
