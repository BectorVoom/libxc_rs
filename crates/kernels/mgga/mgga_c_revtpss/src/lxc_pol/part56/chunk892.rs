//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 892/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk892<F: Float>(t1937: F, t34446: F, t7586: F, t7735: F, t1936: F, t29427: F, t7741: F, t7901: F, t8764: F, t2042: F, t8245: F, t2170: F, t7950: F, t7953: F, t8142: F, t8441: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34447 = t34446 * t1937;
    let t34449 = t7586 * t7735;
    let t34453 = t29427 * t1936;
    let t34455 = t34446 * t1936;
    let t34457 = t7586 * t7741;
    let t34464 = t8764 * t7901;
    let t34481 = t8245 * t2042;
    let t34483 = t2170 * t7950;
    let t34485 = t2170 * t7953;
    let t34866 = t8441 * t8142;
    (t34447, t34449, t34453, t34455, t34457, t34464, t34481, t34483, t34485, t34866)
}
