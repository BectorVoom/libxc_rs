//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 622/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk622<F: Float>(t1770: F, t5900: F, t4042: F, t789: F, t2236: F, t73: F, t2432: F, t707: F, t23: F, t342: F, t2377: F, t3537: F, t1212: F, t2381: F, t4433: F, t4434: F) -> (F, F, F, F, F, F, F, F) {
    let t5901 = t5900 * t1770;
    let t5903 = t789 * t4042;
    let t5934 = t73 * t2236;
    let t5937 = t707 * t2432;
    let t5939 = t342 * t23;
    let t5953 = t3537 * t2377;
    let t5958 = t1212 * t2381;
    let t5961 = -t4433 - t4434;
    (t5901, t5903, t5934, t5937, t5939, t5953, t5958, t5961)
}
