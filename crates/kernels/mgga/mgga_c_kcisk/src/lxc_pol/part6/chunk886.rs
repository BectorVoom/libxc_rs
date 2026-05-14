//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 886/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk886<F: Float>(t30402: F, t30474: F, t67: F, t2152: F, t8010: F, t13776: F, t7831: F) -> (F, F, F, F) {
    let t30476 = t67 * (t30402 + t30474);
    let t30489 = t8010 * t2152;
    let t30490 = t13776 * t30489;
    let t30494 = t2152 * t7831;
    (t30476, t30489, t30490, t30494)
}
