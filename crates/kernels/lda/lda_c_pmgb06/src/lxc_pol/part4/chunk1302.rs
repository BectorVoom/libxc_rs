//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1302/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1302<F: Float>(t15223: F, t36: F, t506: F, t1476: F, t16003: F, t16359: F, t2909: F, t103: F, t13382: F, t14110: F, t14150: F, t15200: F, t15548: F, t17070: F, t525: F, t9577: F, t9956: F, t9958: F, t9981: F, t9986: F) -> (F, F, F, F) {
    let t17107 = t36 * t506 * t15223;
    let t17110 = t36 * t1476 * t16003;
    let t17113 = t36 * t2909 * t16359;
    let t17121 = -F::new(0.007407407407407408) * t9956 - F::new(0.0024691358024691358) * t9958 - F::new(0.047988888888888886) * t13382 + t9981 + F::new(0.03199259259259259) * t9577 + t9986 + F::new(0.023703703703703703) * t15548 * t14110 * t17070 - F::new(0.017777777777777778) * t14150 + F::new(0.8638) * t17107 - F::new(0.8638) * t17110 + F::new(0.47988888888888886) * t17113 - F::new(0.04) * t103 * t525 * t15200 + F::new(0.16) * t103 * t525 * t15223;
    (t17107, t17110, t17113, t17121)
}
