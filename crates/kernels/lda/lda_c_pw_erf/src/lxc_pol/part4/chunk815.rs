//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 815/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk815<F: Float>(t1729: F, t452: F, t454: F, t1872: F, t2765: F, t1184: F, t780: F, t483: F, t1187: F, t169: F, t1891: F, t301: F, t717: F, t143: F, t145: F, t2767: F, t279: F, t2880: F, t2897: F, t2903: F, t2906: F, t2932: F, t2935: F, t2937: F, t296: F, t3203: F, t405: F, t4122: F, t4125: F, t4129: F, t4132: F, t4136: F, t4140: F, t4144: F, t5548: F, t5718: F, t5745: F, t5750: F, t5779: F, t5783: F, t5902: F, t5920: F) -> (F, F, F, F, F, F, F) {
    let t5924 = t1729 * t452 * t454;
    let t5925 = t2765 * t1872;
    let t5931 = t1184 * t780;
    let t5932 = t5931 * t483;
    let t5933 = t5932 * t1187;
    let t5941 = 0.10809180959278285 * t169 * t717 * t1891 * t301;
    let t5942 = 0.39633663517353707 * t3203 + (0.31995040645307626 * t5745 + 0.05332506774217938 * t145 * t5718 - t5750 - 0.10665013548435875 * t2937 + 0.6399008129061525 * t2935 + t2880 - 0.06367133154935875 * t2906 - t2932 + t2897 - 0.031835665774679375 * t2903 + t5779) * t296 - 6.0 * t5783 * t2767 + (t5902 + t5920) * t279 + 12.0 * t5924 * t5925 + 3.0 * t405 * t143 * t5548 - 1.82185769317151e-05 * t5933 - 0.0002905674151788692 * t4122 - 0.0011622696607154768 * t4125 - t4129 + 0.002711962541669446 * t4132 + t4136 - t4140 - t4144 - t5941;
    (t5924, t5925, t5931, t5932, t5933, t5941, t5942)
}
