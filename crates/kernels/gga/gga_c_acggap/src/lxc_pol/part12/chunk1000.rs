//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1000/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1000<F: Float>(t1460: F, t1992: F, t2095: F, t30225: F, t532: F, t1569: F, t7605: F, t2001: F, t5237: F, t13364: F, t31115: F, t35633: F, t1526: F, t2020: F, t2016: F, t8747: F) -> (F, F, F, F, F, F, F) {
    let t36367 = t2095 * t1992 * t1460;
    let t36370 = t30225 * t532;
    let t36372 = t7605 * t1569;
    let t36374 = t2001 * t5237;
    let t36377 = t31115 * t13364 * t35633;
    let t36380 = t2020 * t1526;
    let t36382 = t2016 * t8747;
    (t36367, t36370, t36372, t36374, t36377, t36380, t36382)
}
