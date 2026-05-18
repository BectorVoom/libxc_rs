//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 897/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk897<F: Float>(t1967: F, t7763: F, t7701: F, t381: F, t7636: F, t7461: F, t7637: F, t7770: F, t13716: F, t577: F, t584: F, t1072: F, t167: F, t7322: F) -> (F, F, F, F, F, F, F, F) {
    let t30584 = t1967 * t7763;
    let t30586 = t1967 * t7701;
    let t30589 = t381 * t7636;
    let t30590 = t30589 * t7461;
    let t30592 = t7637 * t7770;
    let t30594 = t13716 * t577;
    let t30595 = t30594 * t584;
    let t30598 = t7322 * t167 * t1072;
    (t30584, t30586, t30589, t30590, t30592, t30594, t30595, t30598)
}
