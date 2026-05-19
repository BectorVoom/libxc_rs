//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 771/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk771<F: Float>(t2595: F, t6892: F, t168: F, t5389: F, t2591: F, t1034: F, t5391: F, t2583: F, t5221: F, t1702: F, t2587: F, t1025: F, t5264: F) -> (F, F, F, F, F, F, F) {
    let t6894 = F::cast_from(0.40015750243531754508e-2_f64) * t6892 * t2595;
    let t6895 = t5389 * t168;
    let t6896 = t6895 * t2591;
    let t6897 = t1034 * t5391;
    let t6914 = F::new(7.0) / F::new(24.0) * t5221 * t2583;
    let t6928 = F::new(7.0) / F::new(72.0) * t1702 * t2587;
    let t6933 = t5264 * t1025;
    (t6894, t6895, t6896, t6897, t6914, t6928, t6933)
}
