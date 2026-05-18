//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 705/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk705<F: Float>(t7251: F, t7258: F, t7261: F, t7268: F, t7904: F, t7906: F, t7908: F) -> F {
    let t7910 = -t7251 - t7904 / F::new(48.0) - t7258 + t7261 - F::new(0.42874018118069736972e-3) * t7906 - t7268 - F::new(0.17149607247227894789e-2) * t7908;
    t7910
}
