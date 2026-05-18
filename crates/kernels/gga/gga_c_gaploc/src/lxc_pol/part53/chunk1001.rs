//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1001/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1001<F: Float>(t38486: F, t901: F, t13792: F, t4379: F, t12000: F, t1429: F, t2365: F, t2366: F, t47953: F, t6963: F, t6964: F, t13801: F, t1641: F) -> (F, F, F, F, F) {
    let t47978 = t38486 * t901;
    let t47980 = t4379 * t13792;
    let t47984 = t1429 * t2365 * t2366 * t12000;
    let t47987 = t6963 * t6964 * t47953;
    let t47989 = t1641 * t13801;
    (t47978, t47980, t47984, t47987, t47989)
}
