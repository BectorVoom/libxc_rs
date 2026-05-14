//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 529/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk529<F: Float>(t1218: F, t1466: F, t1479: F, t1506: F, t301: F, t6215: F, t6216: F, t6963: F, t6967: F, t6972: F, t7024: F, t7028: F, t7097: F, t7110: F, t7115: F, t7125: F, t7129: F, t7131: F) -> (F,) {
    let t7137 = t6963 * t1479 / 6.0 - t6215 - t6216 * t6967 / 18.0 - t1466 * t6972 / 3.0 + t1466 * t7024 / 6.0 + t1466 * t7028 / 6.0 - t1218 * t1506 - t301 * t7129 + 2.0 * t7131 - 2.0 * t7097 - 2.0 * t7110 + 4.0 * t7115 - 2.0 * t7125;
    (t7137,)
}
