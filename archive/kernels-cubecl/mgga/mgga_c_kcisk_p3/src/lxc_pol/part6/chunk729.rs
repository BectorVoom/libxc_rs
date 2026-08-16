//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 729/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk729<F: Float>(t1319: F, t6174: F, t301: F, t342: F, t969: F, t119: F, t416: F, t1337: F, t142: F, t10: F, t3529: F, t1265: F, t4125: F) -> (F, F, F, F, F, F, F) {
    let t13504 = t6174 * t1319;
    let t13522 = t342 * t969 * t301;
    let t13523 = F::cast_from(0.55403703703703703703e-1_f64) * t13522;
    let t13524 = t119 * t416;
    let t13528 = t142 * t1337;
    let t13538 = t10 * t3529;
    let t13561 = F::cast_from(1.0_f64) / t4125 / t1265;
    (t13504, t13522, t13523, t13524, t13528, t13538, t13561)
}
