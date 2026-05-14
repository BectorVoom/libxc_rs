//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1263/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1263<F: Float>(t38974: F, t701: F, t1457: F, t2004: F, t28585: F, t33315: F, t33317: F, t33319: F, t33321: F, t33325: F, t33328: F, t33335: F, t33338: F, t33351: F, t33353: F, t33356: F, t33359: F, t33363: F, t33365: F) -> (F, F) {
    let t39107 = t38974 * t701;
    let t39111 = t33315 + t33317 + t33319 + t33321 - t33325 + t33328 - t33335 - t33338 - t28585 + 0.71500979903700853338e0 * t2004 * t1457 * t39107 - t33351 - t33353 + t33356 - t33359 - t33363 - t33365;
    (t39107, t39111)
}
