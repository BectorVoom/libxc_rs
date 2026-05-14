//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 465/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk465<F: Float>(t221: F, t446: F, t6182: F, t1190: F, t1891: F, t1895: F, t1870: F, t4569: F, t1835: F, t6: F, t1515: F, t1839: F, t5672: F, t1392: F, t1516: F, t1430: F, t1475: F) -> (F, F, F, F, F, F, F, F) {
    let t6184 = t221 * t6182 * t446;
    let t6188 = t1190 * t1891;
    let t6190 = t1190 * t1895;
    let t6192 = t4569 * t1870;
    let t6194 = t6 * t1835;
    let t6196 = t1515 * t6194 * t446;
    let t6199 = t6 * t1839;
    let t6201 = t5672 * t6199 * t446;
    let t6205 = t1515 * t1516 * t1392;
    let t6210 = t221 * t1475 * t1430;
    (t6184, t6188, t6190, t6192, t6196, t6201, t6205, t6210)
}
