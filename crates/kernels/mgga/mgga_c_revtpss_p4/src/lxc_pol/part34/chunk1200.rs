//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1200/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1200<F: Float>(t11874: F, t27492: F, t11970: F, t1973: F, t11858: F, t11926: F, t25516: F, t11940: F, t1972: F, t11735: F, t1968: F, t11772: F, t25515: F) -> (F, F, F, F, F, F, F) {
    let t93548 = t11874 * t27492;
    let t93611 = F::cast_from(0.1270341277572436651e-3_f64) * t1973 * t11970;
    let t93658 = t11858 * t27492;
    let t93667 = t11926 * t25516;
    let t93725 = t11940 * t1972;
    let t93750 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t1968 * t11735;
    let t93751 = t25515 * t11772;
    (t93548, t93611, t93658, t93667, t93725, t93750, t93751)
}
