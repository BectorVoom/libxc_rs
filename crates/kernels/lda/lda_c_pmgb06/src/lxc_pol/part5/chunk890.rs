//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 890/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk890<F: Float>(t1423: F, t6250: F, t13726: F, t806: F, t2007: F, t5220: F, t2012: F, t5210: F, t801: F, t2481: F, t3220: F, t6241: F, t6245: F, t2489: F, t3226: F, t1447: F, t6292: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17984 = t1423 * t6250;
    let t17991 = t13726 * t806;
    let t17993 = t5220 * t2007;
    let t17996 = t801 * t5210 * t2012;
    let t18002 = t3220 * t2481;
    let t18004 = t1423 * t6241;
    let t18006 = t1423 * t6245;
    let t18008 = t3226 * t2489;
    let t18010 = t1447 * t6292;
    (t17984, t17991, t17993, t17996, t18002, t18004, t18006, t18008, t18010)
}
