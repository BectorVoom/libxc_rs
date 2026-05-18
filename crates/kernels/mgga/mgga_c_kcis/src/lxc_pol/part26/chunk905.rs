//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 905/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk905<F: Float>(t21125: F, t3883: F, t26: F, t11462: F, t21130: F, t11408: F, t11409: F, t16046: F, t16052: F, t16183: F, t16184: F, t21186: F, t21188: F, t21193: F, t21196: F, t21206: F, t21209: F, t21212: F, t21234: F, t21237: F, t21240: F, t21243: F) -> (F, F, F) {
    let t21245 = t3883 * t21125;
    let t21246 = t26 * t21245;
    let t21248 = t11462 * t21130;
    let t21249 = t26 * t21248;
    let t21267 = -t11408 - F::new(4.0) / F::new(27.0) * t11409 - F::new(8.0) / F::new(27.0) * t16046 + t16183 - t16184 - F::new(4.0) / F::new(9.0) * t16052 + F::new(2.0) / F::new(27.0) * t21186 - F::new(10.0) / F::new(27.0) * t21237 + F::new(4.0) / F::new(3.0) * t21234 + F::new(8.0) / F::new(9.0) * t21240 - F::new(2.0) / F::new(9.0) * t21188 - F::new(2.0) * t21243 - F::new(8.0) / F::new(3.0) * t21206 + t21196 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t21209 + F::new(2.0) / F::new(3.0) * t21212 - t21193 / F::new(3.0);
    (t21246, t21249, t21267)
}
