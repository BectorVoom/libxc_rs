//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1184/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1184<F: Float>(t286: F, t69: F, t1262: F, t167: F, t1851: F, t3532: F, t1008: F, t9985: F, t4977: F, t26391: F, t26399: F, t26401: F) -> (F, F, F, F, F, F, F) {
    let t61287 = t69 * t286;
    let t67957 = t1262 * t167;
    let t67966 = t1851 * t3532;
    let t71742 = t9985 * t1008;
    let t71743 = t71742 * t4977;
    let t91769 = F::cast_from(18.0_f64) * t26391;
    let t91772 = F::cast_from(6.0_f64) * t26399;
    let t91773 = F::cast_from(12.0_f64) * t26401;
    (t61287, t67957, t67966, t71743, t91769, t91772, t91773)
}
