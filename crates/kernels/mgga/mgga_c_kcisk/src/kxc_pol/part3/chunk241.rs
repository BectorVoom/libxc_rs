//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 241/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk241<F: Float>(t227: F, t1060: F, t229: F, t1059: F, t44: F, t247: F, t242: F, t819: F, t821: F, t825: F, t827: F, t250: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t1063 = piecewise3::<F>(t228, F::new(0.0), F::new(4.0) / F::new(3.0) * t229 * t1060);
    let t1065 = (t1059 + t1063) * t44;
    let t1070 = t247 * t247;
    let t1071 = F::new(1.0) / t1070;
    let t1072 = t242 * t1071;
    let t1077 = -F::new(0.1176575e1) * t819 - F::new(0.516475e0) * t821 - F::new(0.2103875e0) * t825 - F::new(0.104195e0) * t827;
    let t1078 = F::new(1.0) / t250;
    (t1065, t1070, t1071, t1072, t1077, t1078)
}
