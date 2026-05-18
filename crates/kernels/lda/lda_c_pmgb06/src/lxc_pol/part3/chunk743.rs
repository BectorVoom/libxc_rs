//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 743/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk743<F: Float>(t5049: F, t1547: F, t814: F, t132: F, t2998: F, t3007: F, t4070: F, t4079: F, t4082: F, t4089: F, t4091: F, t4973: F, t4977: F, t4981: F, t4983: F, t5043: F, t5046: F, t5048: F) -> (F, F, F, F, F) {
    let t5050 = t5049 / F::new(135.0);
    let t5051 = t1547 * t814;
    let t5052 = t132 * t5051;
    let t5053 = t5052 / F::new(135.0);
    let t5054 = F::new(2.0) / F::new(45.0) * t2998;
    let t5057 = -t4973 - t4977 - t4981 - t4983 - t5043 - t5046 - t5048 - t5050 - t5053 - t5054 + t3007 + t4070 + t4079 + t4082 + t4089 / F::new(3.0) + F::new(0.06077777777777778) * t4091;
    (t5050, t5051, t5053, t5054, t5057)
}
