//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1814/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1814<F: Float>(t73350: F, t48225: F, t85895: F, t48227: F, t73360: F, t48243: F, t39483: F, t39520: F, t39528: F, t39531: F, t39747: F, t46972: F, t46980: F) -> (F, F, F, F, F, F, F) {
    let t91958 = F::new(6.0) * t73350;
    let t91959 = F::new(48.0) * t48225;
    let t91960 = F::cast_from(0.23392894490538584828e1_f64) * t85895;
    let t91961 = F::new(240.0) * t48227;
    let t91962 = F::new(48.0) * t73360;
    let t91963 = F::new(4.0) * t48243;
    let t91964 = t91958 - t46972 - t39483 - t91959 + t39520 - t91960 + t91961 - t39528 - t91962 + t39531 + t91963 + t46980 + t39747;
    (t91958, t91959, t91960, t91961, t91962, t91963, t91964)
}
