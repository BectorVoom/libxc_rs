//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 750/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk750<F: Float>(t1049: F, t1617: F, t3179: F, t687: F, t2011: F, t1461: F, t4043: F, t1030: F, t3141: F, t5059: F, t1044: F, t1971: F) -> (F, F, F, F, F, F, F, F) {
    let t8610 = t1049 * t1617;
    let t8613 = t3179 * t687;
    let t8616 = t1049 * t2011;
    let t8619 = t1461 * t4043;
    let t8620 = t1030 * t8619;
    let t8621 = t3141 * t5059;
    let t8622 = t8620 * t8621;
    let t8624 = t1971 * t1044;
    (t8610, t8613, t8616, t8619, t8620, t8621, t8622, t8624)
}
