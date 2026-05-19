//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 753/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk753<F: Float>(t110: F, t15363: F, t15401: F, t67: F, t10: F, t107: F, t119: F, t142: F, t3020: F, t64: F, t903: F, t918: F, t41: F) -> F {
    let t111 = t110 < -F::new(0.66725e-1);
    let t15403 = t67 * (t15363 + t15401);
    let t15417 = piecewise3::<F>(t111, F::new(0.0), F::new(10.0) / F::new(9.0) * t64 * t15403 * t10 - F::new(10.0) / F::new(9.0) * t64 * t3020 * t142 + F::new(40.0) / F::new(27.0) * t64 * t903 * t119 - F::new(280.0) / F::new(243.0) * t64 * t107 * t918);
    let t15418 = t15417 * t41;
    t15418
}
