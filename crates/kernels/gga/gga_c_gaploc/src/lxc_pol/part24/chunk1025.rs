//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1025/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1025<F: Float>(t4260: F, t883: F, t6490: F, t6525: F, t1436: F, t9544: F, t1538: F, t20395: F, t6583: F, t20481: F, t21414: F, t123: F, t6393: F, t587: F, t912: F, t21071: F, t901: F) -> (F, F, F, F, F, F, F, F) {
    let t30204 = t4260 * t883;
    let t30207 = 0.94850022118920498664e-2 * t6525 * t30204 * t6490;
    let t30246 = t1436 * t9544;
    let t30247 = 0.1022478025437886658e1 * t30246;
    let t30250 = t6583 * t1538 * t883 * t20395;
    let t30251 = 0.76685851907841499352e0 * t30250;
    let t30253 = 0.59584149919750711116e-1 * t20481 * t21414;
    let t30258 = t6393 * t123 * t883;
    let t30260 = t587 * t912 * t30258;
    let t30261 = 0.38342925953920749676e0 * t30260;
    let t30263 = 0.29792074959875355558e-1 * t21071 * t901;
    (t30204, t30207, t30247, t30251, t30253, t30258, t30261, t30263)
}
