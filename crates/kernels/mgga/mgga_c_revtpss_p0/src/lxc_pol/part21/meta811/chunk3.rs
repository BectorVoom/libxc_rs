//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2965/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2965<F: Float>(t11672: F, t15682: F, t12078: F, t53552: F, t16183: F, t73: F, t42793: F, t4892: F, t4895: F, t15951: F, t3127: F, t3172: F) -> (F, F, F, F, F) {
    let t54014 = t11672 * t15682;
    let t54023 = t12078 * t53552;
    let t54026 = t16183 * t73;
    let t54036 = t4892 * t42793 * t4895;
    let t54037 = F::cast_from(0.28582678745379824648e-3_f64) * t54036;
    let t54039 = t3127 * t3172 * t15951;
    (t54014, t54023, t54026, t54037, t54039)
}
