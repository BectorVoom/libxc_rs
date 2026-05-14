//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 773/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk773<F: Float>(t717: F, t780: F, t1138: F, t1597: F, t1124: F, t483: F, t485: F, t1904: F, t473: F, t1131: F, t1910: F, t4168: F, t4172: F, t4175: F, t4246: F, t4254: F, t4260: F, t4261: F, t4265: F, t4268: F, t4270: F, t4272: F, t4275: F, t4276: F, t4279: F) -> (F, F, F, F, F, F, F, F) {
    let t5466 = t717 * t780;
    let t5468 = t5466 * t1138 * t1597;
    let t5470 = t1124 * t780;
    let t5472 = t5470 * t483 * t485;
    let t5474 = t473 * t1904;
    let t5477 = 0.003950778065781896 * t5474 * t483 * t485;
    let t5479 = t1910 * t1131 * t485;
    let t5487 = 0.013169260219272987 * t4168 + t4172 + t4175 - 0.0004954275694490498 * t5468 + 0.006584630109636494 * t5472 - t5477 - 0.003950778065781896 * t5479 - 0.12602162889256446 * t4276 - t4279 + t4254 - t4260 - 0.06301081444628223 * t4261 - t4265 - 0.031505407223141116 * t4268 + 0.031505407223141116 * t4270 + 0.12602162889256446 * t4272 + t4275 + 0.008980675507690957 * t4246;
    (t5466, t5468, t5470, t5472, t5474, t5477, t5479, t5487)
}
