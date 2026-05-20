//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1644/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1644<F: Float>(t1196: F, t16682: F, t12500: F, t5205: F, t1733: F, t3385: F, t3433: F, t3302: F, t5332: F, t1214: F, t5333: F, t1716: F, t2435: F) -> (F, F, F, F, F, F, F) {
    let t16684 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t16682;
    let t16685 = t5205 * t12500;
    let t16687 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t16685;
    let t16688 = t1733 * t3385;
    let t16690 = F::new(6.0) * t3433 * t16688;
    let t16695 = t5332 * t3302;
    let t16696 = t5333 * t1214;
    let t16697 = t16695 * t16696;
    let t16706 = t2435 * t1716;
    (t16684, t16687, t16690, t16695, t16696, t16697, t16706)
}
