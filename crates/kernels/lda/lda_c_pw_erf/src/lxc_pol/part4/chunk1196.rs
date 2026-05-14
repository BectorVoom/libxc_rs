//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1196/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1196<F: Float>(t10030: F, t6756: F, t16554: F, t352: F, t4506: F, t4515: F, t4522: F, t2337: F, t549: F, t3974: F, t5160: F, t5166: F, t13389: F, t17628: F, t17632: F, t17636: F, t17640: F, t17642: F, t17644: F, t17648: F, t17653: F, t17656: F, t17658: F, t17663: F) -> (F, F, F, F, F, F, F) {
    let t17664 = t10030 * t6756;
    let t17665 = 64.0 / 135.0 * t17664;
    let t17666 = t16554 * t352;
    let t17669 = 16.0 / 45.0 * t4506 * t4515 * t17666;
    let t17672 = 8.0 / 27.0 * t4506 * t4522 * t17666;
    let t17673 = t2337 * t352;
    let t17674 = t17673 * t549;
    let t17677 = 32.0 / 45.0 * t3974 * t5160 * t17674;
    let t17680 = 16.0 / 27.0 * t3974 * t5166 * t17674;
    let t17681 = 32.0 / 135.0 * t13389;
    let t17682 = t17628 - t17632 + t17636 - t17640 - t17642 + t17644 + t17648 - t17653 - t17656 - t17658 - t17663 - t17665 + t17669 - t17672 - t17677 + t17680 - t17681;
    (t17665, t17669, t17672, t17677, t17680, t17681, t17682)
}
