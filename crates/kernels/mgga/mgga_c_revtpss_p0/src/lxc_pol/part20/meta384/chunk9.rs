//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1411/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1411<F: Float>(t10489: F, t11084: F, t14375: F, t198: F, t2403: F, t2430: F, t262: F, t39989: F, t40128: F, t40131: F, t40133: F, t40137: F, t40140: F, t40142: F, t40144: F, t40146: F, t40149: F, t40151: F, t4541: F, t775: F) -> F {
    let t41185 = F::new(24.0) * t10489 * t262 * t4541 * t775 - F::new(18.0) * t11084 * t2403 * t2430 + F::new(36.0) * t14375 * t198 * t2430 - t39989 + t40128 - t40131 - t40133 - t40137 + t40140 + t40142 + t40144 + t40146 + t40149 + t40151;
    t41185
}
