//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 673/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk673<F: Float>(t402: F, t5967: F, t2705: F, t4387: F, t4389: F, t4391: F, t2740: F, t4398: F, t4401: F, t4406: F, t4408: F, t4412: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5968 = t5967 * t402;
    let t5969 = F::cast_from(0.5848223397455204_f64) * t5968;
    let t5970 = F::cast_from(0.010843580882781523_f64) * t2705;
    let t5971 = F::cast_from(0.0004883081343134176_f64) * t4387;
    let t5972 = F::cast_from(1.169644679491041_f64) * t4389;
    let t5973 = F::cast_from(34.631511798751724_f64) * t4391;
    let t5974 = F::cast_from(0.5848223397455204_f64) * t2740;
    let t5975 = F::cast_from(0.021687161765563047_f64) * t4398;
    let t5976 = F::new(24.0) * t4401;
    let t5977 = F::new(2.0) * t4406;
    let t5978 = F::new(40.0) * t4408;
    let t5979 = F::cast_from(2.339289358982082_f64) * t4412;
    (t5968, t5969, t5970, t5971, t5972, t5973, t5974, t5975, t5976, t5977, t5978, t5979)
}
