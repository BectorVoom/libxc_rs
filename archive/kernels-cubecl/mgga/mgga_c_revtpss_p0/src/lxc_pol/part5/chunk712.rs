//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 712/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk712<F: Float>(t4719: F, t983: F, t1642: F, t3022: F, t1633: F, t2986: F, t974: F, t981: F, t4707: F, t964: F, t973: F, t3011: F) -> (F, F, F, F, F, F, F, F) {
    let t4721 = F::cast_from(0.5848223622634646207e0_f64) * t4719 * t983;
    let t4723 = F::cast_from(0.5848223622634646207e0_f64) * t3022 * t1642;
    let t4724 = t2986 * t1633;
    let t4725 = t4724 * t974;
    let t4727 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t4725;
    let t4729 = t964 * t4707 * t973;
    let t4731 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t4729;
    let t4732 = t3011 * t1633;
    (t4721, t4723, t4724, t4725, t4727, t4729, t4731, t4732)
}
