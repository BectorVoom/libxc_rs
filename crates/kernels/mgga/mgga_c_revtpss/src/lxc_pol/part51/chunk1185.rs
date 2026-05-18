//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1185/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1185<F: Float>(t33976: F, t7235: F, t119578: F, t28067: F, t28167: F, t37972: F, t5627: F, t28177: F, t8568: F, t34258: F, t7003: F, t2014: F, t49575: F, t8599: F) -> (F, F, F, F, F, F) {
    let t127335 = t7235 * t33976;
    let t127336 = t119578 * t28067;
    let t127340 = F::new(6.0) * t28167 * t37972 * t5627;
    let t127341 = t8568 * t28177;
    let t127346 = F::new(4.0) * t34258 * t7003;
    let t127349 = F::new(2.0) * t2014 * t8599 * t49575;
    (t127335, t127336, t127340, t127341, t127346, t127349)
}
