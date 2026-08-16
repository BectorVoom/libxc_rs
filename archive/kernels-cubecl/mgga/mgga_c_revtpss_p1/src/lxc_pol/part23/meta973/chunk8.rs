//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3306/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3306<F: Float>(t1450: F, t23059: F, t1868: F, t39528: F, t39531: F, t4139: F, t48234: F, t48236: F, t48241: F, t48244: F, t75389: F, t85896: F, t85897: F, t85898: F, t85899: F) -> (F, F) {
    let t86731 = t23059 * t1450;
    let t86741 = F::cast_from(9.0_f64) * t1868 * t4139 * t75389 - t39528 + t39531 + t48234 + t48236 + t48241 - t48244 - t85896 + t85897 - t85898 + t85899;
    (t86731, t86741)
}
