//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1273/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1273<F: Float>(t1087: F, t93596: F, t11744: F, t7132: F, t1068: F, t11748: F, t11806: F, t11845: F, t11855: F, t11983: F, t25569: F, t25577: F, t27536: F, t3097: F, t3101: F, t3177: F, t93573: F, t93579: F, t93583: F, t93585: F, t93592: F) -> F {
    let t93597 = t1087 * t93596;
    let t93602 = t7132 * t11744;
    let t93606 = -F::new(0.85748036236139473944e-3) * t93573 + F::new(0.14291339372689912324e-2) * t7132 * t11983 + F::new(0.1270341277572436651e-2) * t7132 * t11855 + F::new(0.11433071498151929859e-2) * t93579 + F::new(0.25724410870841842183e-2) * t27536 * t11806 + F::new(0.57165357490759649295e-3) * t93583 + F::new(0.95275595817932748825e-3) * t93585 + F::new(0.85748036236139473944e-3) * t25569 * t3177 + F::new(0.28582678745379824648e-3) * t7132 * t11845 + F::new(0.28963781128651555642e-1) * t93592 * t1068 - F::new(0.91464571985215438873e-2) * t93597 * t3097 + F::new(0.91464571985215438873e-2) * t25577 * t3101 - F::new(0.11433071498151929859e-2) * t93602 + F::new(0.17149607247227894789e-2) * t7132 * t11748;
    t93606
}
