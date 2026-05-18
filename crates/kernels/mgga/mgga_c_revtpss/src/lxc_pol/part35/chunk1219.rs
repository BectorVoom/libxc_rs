//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1219/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1219<F: Float>(t103404: F, t103422: F, t103432: F, t110525: F, t110527: F, t110542: F, t110544: F, t110572: F, t110584: F, t110591: F, t23384: F, t23404: F, t25317: F, t28394: F, t29698: F, t6048: F, t6072: F, t7070: F, t7403: F, t7997: F, t8016: F) -> F {
    let t115637 = -F::new(0.43368140941025997312e-1) * t110525 + F::new(0.77108554593144223218e-1) * t110527 - F::new(0.51405703062096148812e-1) * t103404 - F::new(0.78062653693846795158e1) * t7070 * t25317 * t7997 * t6048 + F::new(0.39512695097613069591e1) * t7403 * t23404 + F::new(0.51405703062096148812e-1) * t103422 - F::new(0.21684070470512998656e-1) * t110542 + F::new(0.38554277296572111609e-1) * t110544 - F::new(0.10281140612419229762e0) * t103432 + F::new(0.13010442282307799194e0) * t110572 - F::new(0.32927245914677557992e-1) * t110584 - F::new(0.19756347548806534796e1) * t28394 * t6072 + F::new(0.32927245914677557992e-1) * t110591 - F::new(0.13010442282307799193e1) * t29698 * t8016 - F::new(0.65854491829355115987e0) * t7403 * t23384;
    t115637
}
