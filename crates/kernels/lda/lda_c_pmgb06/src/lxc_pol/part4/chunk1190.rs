//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1190/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1190<F: Float>(t18002: F, t1423: F, t6241: F, t6245: F, t2489: F, t3226: F, t1447: F, t6292: F, t1420: F, t6124: F, t1972: F, t5313: F, t441: F, t6673: F, t439: F, t445: F) -> (F, F, F, F, F, F, F, F) {
    let t18003 = 4.0 / 135.0 * t18002;
    let t18004 = t1423 * t6241;
    let t18005 = 4.0 / 135.0 * t18004;
    let t18006 = t1423 * t6245;
    let t18007 = 4.0 / 135.0 * t18006;
    let t18008 = t3226 * t2489;
    let t18009 = 8.0 / 135.0 * t18008;
    let t18010 = t1447 * t6292;
    let t18011 = 8.0 / 135.0 * t18010;
    let t18013 = 2.0 / 45.0 * t1420 * t6124;
    let t18015 = 4.0 / 45.0 * t1972 * t5313;
    let t18016 = t441 * t6673;
    let t18019 = 2.0 / 45.0 * t439 * t18016 * t445;
    (t18003, t18005, t18007, t18009, t18011, t18013, t18015, t18019)
}
