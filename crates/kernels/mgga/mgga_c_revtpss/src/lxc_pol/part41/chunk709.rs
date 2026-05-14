//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 709/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk709<F: Float>(t4719: F, t983: F, t1642: F, t3022: F, t1633: F, t2986: F, t974: F, t981: F, t4707: F, t964: F, t973: F, t3011: F, t3014: F, t972: F, t2848: F, t3037: F, t4571: F, t4576: F, t4581: F, t4585: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4721 = 0.5848223622634646207e0 * t4719 * t983;
    let t4723 = 0.5848223622634646207e0 * t3022 * t1642;
    let t4724 = t2986 * t1633;
    let t4725 = t4724 * t974;
    let t4727 = 0.11696447245269292414e1 * t981 * t4725;
    let t4729 = t964 * t4707 * t973;
    let t4731 = 0.5848223622634646207e0 * t981 * t4729;
    let t4732 = t3011 * t1633;
    let t4733 = t3014 * t972;
    let t4734 = t4732 * t4733;
    let t4736 = 0.17315859105681463759e2 * t981 * t4734;
    let t4742 = t3037 + 0.27777777777777777778e-2 * t2848 + 0.27777777777777777778e-2 * t4571 - 0.55555555555555555555e-2 * t4576 + 0.16666666666666666667e-1 * t4581 - 0.83333333333333333333e-2 * t4585;
    (t4721, t4723, t4724, t4725, t4727, t4729, t4731, t4732, t4733, t4734, t4736, t4742)
}
