//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 779/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk779<F: Float>(t120: F, t7913: F, t102: F, t436: F, t3296: F, t7918: F, t7159: F, t7162: F, t2610: F, t763: F, t127: F, t1852: F, t3313: F, t3322: F, t7143: F, t7146: F, t7149: F, t7152: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7924 = t120 * t7913;
    let t7926 = 2.923025 * t102 * t7924;
    let t7927 = t436 * t7913;
    let t7930 = t3296 * t7918;
    let t7933 = t436 * t7918;
    let t7935 = 17.53815 * t102 * t7933;
    let t7940 = 2.923025 * t7159;
    let t7941 = 1.4615125 * t7162;
    let t7947 = 17.53815 * t102 * t763 * t2610;
    let t7948 = -t7926 - 1.46904 * t127 * t7927 - 29.3808 * t127 * t7930 - t7935 - 3.0 / 2.0 * t7143 + t7146 / 2.0 - 8.81424 * t7149 + 2.20356 * t7152 - t7940 + t7941 + t3313 - t3322 + 17.62848 * t127 * t1852 * t2610 + t7947;
    (t7924, t7926, t7927, t7930, t7933, t7935, t7940, t7941, t7947, t7948)
}
