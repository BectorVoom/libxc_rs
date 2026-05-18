//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1343/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1343<F: Float>(t15849: F, t15923: F, t15985: F, t16670: F, t16734: F, t16802: F, t17004: F, t17303: F, t589: F, t1505: F, t5895: F, t1555: F) -> (F, F) {
    let t17306 = t15849 + t15923 + t15985 + t16670 + t16734 + t16802 + t17004 + t17303;
    let t17307 = t17306 * t589;
    let t17308 = t5895 * t1505;
    let t17310 = F::new(2.0) * t17308 * t1555;
    (t17307, t17310)
}
