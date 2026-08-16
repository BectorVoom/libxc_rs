//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1366/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1366<F: Float>(t10475: F, t10478: F, t10472: F, t3131: F, t360: F, t248: F, t2776: F, t3051: F, t1041: F, t3103: F, t3109: F, t3114: F) -> (F, F, F, F, F, F) {
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10489 = t248 * t3051 * t2776;
    let t10490 = t1041 * t10489;
    let t10496 = t3109 * t3103;
    let t10504 = t3114 * t3103;
    (t10480, t10482, t10489, t10490, t10496, t10504)
}
