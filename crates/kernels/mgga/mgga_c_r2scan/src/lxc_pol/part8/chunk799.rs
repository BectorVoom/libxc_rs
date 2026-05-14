//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 799/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk799<F: Float>(t5986: F, t1654: F, t761: F, t2049: F, t597: F, t2061: F, t158: F, t2288: F, t2056: F) -> (F, F, F, F, F, F) {
    let t5987 = 240.0 * t5986;
    let t5998 = t1654 * t761;
    let t6001 = t597 * t2049;
    let t6002 = t2061 * t6001;
    let t6006 = t2288 * t158;
    let t6007 = t2056 * t761;
    (t5987, t5998, t6001, t6002, t6006, t6007)
}
