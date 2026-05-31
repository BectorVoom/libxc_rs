//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 763/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk763<F: Float>(t577: F, t7007: F, t2193: F, t4763: F, t5340: F, t5343: F, t2498: F, t514: F, t185: F, t2076: F, t2137: F, t5365: F, t5373: F, t5380: F, t5399: F, t5411: F, t5423: F, t5871: F, t5872: F, t5874: F, t7001: F, t7006: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7009 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t7007 * t577;
    let t7011 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4763 * t2193;
    let t7014 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t5340;
    let t7015 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t5343;
    let t7016 = t514 * t2498;
    let t7017 = t185 * t7016;
    let t7018 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t7017;
    let t7019 = t2076 * t2137;
    let t7020 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7019;
    let t7021 = -t7001 + t7006 + t7009 - t7011 + t5871 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5872 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5874 - t7014 - t7015 - t5365 + t5373 - t5380 + t5399 + t5411 - t5423 - t7018 + t7020;
    (t7009, t7011, t7014, t7015, t7016, t7017, t7018, t7019, t7020, t7021)
}
