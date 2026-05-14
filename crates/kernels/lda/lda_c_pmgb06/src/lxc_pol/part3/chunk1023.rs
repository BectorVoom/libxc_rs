//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1023/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1023<F: Float>(t10412: F, t1385: F, t14024: F, t1420: F, t1604: F, t1629: F, t1848: F, t1868: F, t1893: F, t1898: F, t2010: F, t2948: F, t3040: F, t3177: F, t439: F, t486: F, t4945: F, t5168: F, t5226: F, t5290: F, t5291: F, t5294: F, t5295: F, t831: F) -> (F,) {
    let t14053 = -2.0 / 15.0 * t14024 + t1848 * t1604 / 5.0 + t831 * t3040 / 5.0 - t486 * t4945 / 10.0 - 2.0 / 15.0 * t2010 * t1385 * t1868 * t1629 - 2.0 / 15.0 * t3177 * t1898 - 4.0 / 15.0 * t1420 * t5226 - t1420 * t5291 / 15.0 - 4.0 / 15.0 * t5168 * t5295 - t439 * t10412 * t1893 / 15.0 - t439 * t2948 * t5290 / 15.0 - 4.0 / 15.0 * t2010 * t2948 * t5294;
    (t14053,)
}
