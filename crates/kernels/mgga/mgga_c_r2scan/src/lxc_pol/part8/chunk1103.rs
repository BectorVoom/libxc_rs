//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1103/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1103<F: Float>(t2097: F, t2105: F, t254: F, t265: F, t6077: F, t8: F, t9: F, t537: F, t6243: F, t481: F, t6363: F, t2183: F, t6148: F, t489: F, t6188: F, t6189: F) -> (F, F, F, F, F) {
    let t19904 = 0.36021350028521610017e1 * t254 * t2097 / t9 / t6077 / t8 * t265 * t2105;
    let t19905 = t6243 * t537;
    let t19978 = t6363 * t481;
    let t20040 = t2183 * t6148;
    let t20090 = t6188 * t6189 * t489;
    (t19904, t19905, t19978, t20040, t20090)
}
