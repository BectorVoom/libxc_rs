//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1167/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1167<F: Float>(t11004: F, t12567: F, t3618: F, t983: F, t11002: F, t3269: F, t40491: F, t986: F, t3262: F, t3263: F, t11621: F, t3275: F, t40687: F) -> (F, F, F, F) {
    let t42953 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t12567 * t11004;
    let t42955 = t3618 * t983;
    let t42956 = t11002 * t42955;
    let t42958 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3269 * t42956;
    let t42959 = t40491 * t986;
    let t42962 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t3263 * t42959;
    let t42965 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t3275 * t40687 * t11621;
    (t42953, t42958, t42962, t42965)
}
