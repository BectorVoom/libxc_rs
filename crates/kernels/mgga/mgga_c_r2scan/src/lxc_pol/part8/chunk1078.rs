//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1078/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1078<F: Float>(t2090: F, t386: F, t462: F, t85: F, t1385: F, t1398: F, t735: F, t1467: F, t4700: F, t4823: F) -> (F, F, F) {
    let t19405 = 0.18989649058080861537e-2 * t462 * t386 * t2090 * t85;
    let t19421 = 0.12842595503380418954e1 * t735 * t1398 * t1385;
    let t19424 = 0.57895126195293126241e3 * t4823 * t4700 * t1467;
    (t19405, t19421, t19424)
}
