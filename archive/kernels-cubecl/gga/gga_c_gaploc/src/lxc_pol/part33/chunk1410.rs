//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1410/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1410<F: Float>(t12270: F, t12272: F, t12277: F, t1960: F, t2208: F, t33952: F, t33955: F, t33958: F, t33963: F, t33966: F, t33968: F, t33970: F, t33973: F, t33974: F, t33977: F, t33979: F, t33980: F, t33982: F, t33988: F, t3749: F, t5549: F, t5552: F, t841: F) -> F {
    let t38906 = F::cast_from(4.0_f64) * t12270 * t1960 * t841 + F::cast_from(4.0_f64) * t12272 * t5552 - t12277 * t2208 - t3749 * t5549 - t33952 + t33955 - t33958 - t33963 - t33966 + t33968 + t33970 - t33973 + t33974 + t33977 + t33979 - t33980 - t33982 + t33988;
    t38906
}
