//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1061/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1061<F: Float>(t8281: F, t8286: F, t8291: F, t11369: F, t11371: F, t11373: F, t1034: F, t2343: F, t40: F, t8303: F, t344: F, t6071: F, t1064: F, t2344: F, t1067: F, t1070: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15443 = 0.010843580882781523 * t8281;
    let t15444 = 0.032530742648344574 * t8286;
    let t15445 = 0.9631889219027824 * t8291;
    let t15446 = 2050.779404201559 * t11369;
    let t15447 = 69.26302359750345 * t11371;
    let t15448 = 2.339289358982082 * t11373;
    let t15450 = t40 * t2343 * t1034;
    let t15452 = 207.78907079251036 * t8303;
    let t15453 = t344 * t6071;
    let t15454 = 8.0 * t15453;
    let t15455 = t1064 * t2344;
    let t15456 = 20.0 * t15455;
    let t15457 = t1067 * t2344;
    let t15458 = 12.0 * t15457;
    let t15459 = t1070 * t2344;
    (t15443, t15444, t15445, t15446, t15447, t15448, t15450, t15452, t15454, t15456, t15458, t15459)
}
