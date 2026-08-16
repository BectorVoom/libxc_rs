//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1292/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1292<F: Float>(t3057: F, t7135: F, t11200: F, t1976: F, t3063: F, t8521: F, t7143: F, t1035: F, t1983: F, t36870: F, t1096: F, t19482: F) -> (F, F, F, F, F, F) {
    let t94023 = t3057 * t7135;
    let t94026 = t11200 * t1976;
    let t94042 = t3063 * t8521;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94064 = t19482 * t1096;
    (t94023, t94026, t94042, t94053, t94063, t94064)
}
