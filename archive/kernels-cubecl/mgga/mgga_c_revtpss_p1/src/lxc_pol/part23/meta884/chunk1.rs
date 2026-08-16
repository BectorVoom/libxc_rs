//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2797/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2797<F: Float>(t1882: F, t5710: F, t2782: F, t4086: F, t543: F, t74973: F, t1398: F, t6888: F, t786: F, t4104: F, t23037: F, t10022: F) -> (F, F, F, F, F, F) {
    let t75198 = t5710 * t1882;
    let t75205 = t2782 * t4086 * t74973 * t543;
    let t75215 = t2782 * t4086 * t6888 * t1398 * t543;
    let t75219 = t2782 * t4086 * t75198 * t543;
    let t75251 = t786 * t4086 * t6888;
    let t75252 = t75251 * t4104;
    let t75267 = t23037 * t1398;
    let t75269 = t2782 * t10022 * t75267;
    (t75205, t75215, t75219, t75251, t75252, t75269)
}
