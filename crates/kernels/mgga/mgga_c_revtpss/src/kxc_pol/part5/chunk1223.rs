//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1223/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1223<F: Float>(t3011: F, t6205: F, t4733: F, t981: F, t15258: F, t4732: F, t4719: F, t4729: F, t19136: F, t19143: F, t19145: F, t19149: F, t19152: F, t19252: F, t19258: F, t19315: F, t19317: F, t19320: F, t19323: F, t19326: F, t19329: F, t19333: F, t19337: F) -> (F, F, F, F) {
    let t19467 = t3011 * t6205;
    let t19468 = t19467 * t4733;
    let t19470 = F::new(0.17315859105681463759e2) * t981 * t19468;
    let t19471 = t4732 * t15258;
    let t19473 = F::new(0.34631718211362927518e2) * t981 * t19471;
    let t19475 = F::new(0.11696447245269292414e1) * t4719 * t4729;
    let t19476 = t19136 + t19143 - t19145 + t19149 + t19152 + t19337 + t19252 + t19258 - t19315 + t19317 + t19320 - t19323 - t19326 - t19329 + t19333 - t19470 - t19473 - t19475;
    (t19470, t19473, t19475, t19476)
}
