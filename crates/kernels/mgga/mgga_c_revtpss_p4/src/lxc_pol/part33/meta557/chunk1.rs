//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1949/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1949<F: Float>(t30004: F, t508: F, t651: F, t7898: F, t7935: F, t2022: F, t6895: F, t25924: F, t1903: F, t7910: F, t7296: F, t6918: F) -> (F, F, F, F, F, F, F, F) {
    let t30005 = t508 * t30004;
    let t30007 = F::new(2.0) * t651 * t30005;
    let t30015 = F::new(2.0) * t7898 * t7935;
    let t30016 = t2022 * t6895;
    let t30017 = t25924 * t30016;
    let t30020 = t7910 * t1903;
    let t30021 = t7296 * t30020;
    let t30031 = t2022 * t6918;
    (t30005, t30007, t30015, t30016, t30017, t30020, t30021, t30031)
}
