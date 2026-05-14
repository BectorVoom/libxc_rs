//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 632/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk632<F: Float>(t402: F, t5967: F, t2705: F, t4387: F, t4389: F, t4391: F, t2740: F, t4398: F, t4401: F, t4406: F, t4408: F, t4412: F, t2325: F, t2953: F, t2329: F, t939: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5968 = t5967 * t402;
    let t5969 = 0.5848223397455204 * t5968;
    let t5970 = 0.010843580882781523 * t2705;
    let t5971 = 0.0004883081343134176 * t4387;
    let t5972 = 1.169644679491041 * t4389;
    let t5973 = 34.631511798751724 * t4391;
    let t5974 = 0.5848223397455204 * t2740;
    let t5975 = 0.021687161765563047 * t4398;
    let t5976 = 24.0 * t4401;
    let t5977 = 2.0 * t4406;
    let t5978 = 40.0 * t4408;
    let t5979 = 2.339289358982082 * t4412;
    let t5982 = t2953 * t2325;
    let t5987 = t939 * t2329;
    (t5968, t5969, t5970, t5971, t5972, t5973, t5974, t5975, t5976, t5977, t5978, t5979, t5982, t5987)
}
