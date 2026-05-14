//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1372/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1372<F: Float>(t11272: F, t2990: F, t3002: F, t3004: F, t3018: F, t4304: F, t4308: F, t5690: F, t5695: F, t5702: F, t5950: F, t7315: F, t8118: F, t8120: F, t8121: F, t8122: F, t8123: F, t8126: F, t8130: F, t8134: F) -> (F,) {
    let t19935 = 16.0 * t5690 - t8118 - 103.89453539625518 * t2990 - t8120 + t8121 - 48.0 * t5695 - 3.2861063072053334 * t5950 + t8122 - t8123 - 0.0003662311007350632 * t3002 + 0.0014649244029402528 * t3004 - t8126 - 0.0014649244029402528 * t5702 + t8130 + 4.107632884006667 * t4304 - 0.8215265768013333 * t4308 - 0.8215265768013333 * t7315 + 6.0 * t3018 + t8134 + t11272;
    (t19935,)
}
