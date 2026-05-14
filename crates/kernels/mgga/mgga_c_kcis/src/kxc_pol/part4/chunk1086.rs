//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1086/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1086<F: Float>(t14664: F, t14667: F, t14670: F, t14671: F, t14672: F, t14674: F, t14676: F, t14679: F, t14682: F, t14685: F, t15093: F, t15108: F, t15790: F, t187: F, t236: F, t233: F) -> (F,) {
    let t15793 = t14664 - t14667 + t14670 - t14671 - t14672 + t14674 - t14676 - t14679 + t14682 + t14685 - t15093 + t187 * (t15108 + t15790);
    let t15794 = t236 * t15793;
    let t15795 = t233 * t15794;
    (t15795,)
}
