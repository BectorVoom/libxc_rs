//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1204/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1204<F: Float>(t26230: F, t9681: F, t94674: F, t7523: F, t94610: F, t96232: F, t96234: F, t96237: F, t96240: F, t96243: F, t96246: F, t96249: F, t96253: F, t96257: F, t96260: F, t96262: F, t96265: F, t96269: F) -> (F, F) {
    let t96271 = t26230 * t9681;
    let t96272 = t94674 * t96271;
    let t96274 = F::new(0.21684070470512998656e-1) * t96232 + F::new(0.77108554593144223218e-1) * t96234 - F::new(0.15421710918628844643e0) * t96237 + F::new(0.15421710918628844643e0) * t96240 - F::new(0.43368140941025997312e-1) * t96243 - F::new(0.51405703062096148812e-1) * t96246 + F::new(0.38554277296572111609e-1) * t96249 - F::new(0.19514881078765566038e-2) * t96253 - t96257 - F::new(0.68549505033305214441e-2) * t96260 - F::new(0.38554277296572111609e-1) * t96262 - F::new(0.10281140612419229762e0) * t96265 + F::new(0.26020884564615598386e1) * t94610 * t7523 - F::new(0.21684070470512998656e-1) * t96269 + F::new(0.13010442282307799194e0) * t96272;
    (t96271, t96274)
}
