//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1056/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1056<F: Float>(t4265: F, t6284: F, t15197: F, t6292: F, t19136: F, t6287: F, t19132: F, t18053: F, t6288: F, t6280: F, t14434: F, t1470: F, t18081: F, t19229: F, t19367: F, t19444: F, t21140: F, t21145: F, t21152: F, t21154: F, t21158: F, t21163: F, t21164: F, t4253: F, t5928: F, t5949: F, t6278: F) -> (F,) {
    let t21168 = 0.35374814814814814814e-1 * t4265 * t6284;
    let t21169 = t15197 * t6292;
    let t21171 = t6287 * t19136;
    let t21174 = t6287 * t19132;
    let t21177 = t18053 * t6288;
    let t21180 = 0.5895802469135802469e-1 * t18053 * t6280;
    let t21181 = 0.26531111111111111111e0 * t6278 * t21140 - 0.9286875e-2 * t4253 * t19367 - 0.232171875e-2 * t21145 * t19229 - 0.9286875e-2 * t4253 * t19444 - 0.1857375e-1 * t14434 * t5949 + 0.5895802469135802469e-2 * t21152 - 0.35374814814814814814e-1 * t21154 - 0.53062222222222222222e-1 * t1470 * t21158 - t21163 + 0.9286875e-2 * t21164 * t5928 - t21168 + 0.88437037037037037036e-1 * t21169 + 0.53062222222222222222e-1 * t6278 * t21171 + 0.21224888888888888888e0 * t18081 * t21174 + 0.70749629629629629629e-1 * t21177 - t21180;
    (t21181,)
}
