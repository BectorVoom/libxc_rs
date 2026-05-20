//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1299/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1299<F: Float>(t300: F, t4682: F, t983: F, t3030: F, t4719: F, t3034: F, t11591: F, t1642: F, t11524: F, t4732: F, t981: F, t2989: F) -> (F, F, F, F, F, F) {
    let t15547 = t300 * t4682;
    let t15549 = F::cast_from(0.11696447245269292414e1_f64) * t15547 * t983;
    let t15551 = F::cast_from(0.5848223622634646207e0_f64) * t4719 * t3030;
    let t15553 = F::cast_from(0.17315859105681463759e2_f64) * t4719 * t3034;
    let t15555 = F::cast_from(0.5848223622634646207e0_f64) * t11591 * t1642;
    let t15556 = t4732 * t11524;
    let t15558 = F::cast_from(0.17315859105681463759e2_f64) * t981 * t15556;
    let t15559 = t4732 * t2989;
    (t15549, t15551, t15553, t15555, t15558, t15559)
}
