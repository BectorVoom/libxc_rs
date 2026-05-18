//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1351/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1351<F: Float>(t25082: F, t49640: F, t8717: F, t25191: F, t7235: F, t2322: F, t25861: F, t13435: F, t7003: F, t25856: F, t25188: F, t7313: F) -> (F, F, F, F, F, F) {
    let t95032 = F::new(9.0) * t25082 * t8717 * t49640;
    let t95036 = F::new(18.0) * t7235 * t25191;
    let t95038 = F::new(12.0) * t2322 * t25861;
    let t95040 = F::new(12.0) * t13435 * t7003;
    let t95042 = F::new(6.0) * t2322 * t25856;
    let t95046 = F::new(3.0) * t25188 * t7313;
    (t95032, t95036, t95038, t95040, t95042, t95046)
}
