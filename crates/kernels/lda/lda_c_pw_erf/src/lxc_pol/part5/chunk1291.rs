//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1291/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1291<F: Float>(t211: F, t514: F, t7514: F, t565: F, t7515: F, t14256: F, t14314: F, t14352: F, t14366: F, t23067: F, t23069: F, t23070: F, t23071: F, t23073: F, t23076: F, t23077: F, t23078: F) -> (F, F, F) {
    let t23080 = t211 * t514 * t7514;
    let t23081 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t23080;
    let t23083 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t565 * t7515;
    let t23084 = -t14256 - t23067 - t23069 - t23070 + t23071 - t23073 + t14314 - t14352 - t23076 - t23077 + t23078 - t23081 - t23083 + t14366;
    (t23081, t23083, t23084)
}
