//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 768/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk768<F: Float>(t25365: F, t7060: F, t25296: F, t7064: F, t2435: F, t7015: F, t251: F, t786: F, t1032: F, t2769: F, t233: F, t122: F, t1949: F, t72: F, t2466: F, t1955: F, t25308: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25366 = t25365 * t7060;
    let t25368 = t7064 * t25296;
    let t25371 = 0.73171657588172351096e-2 * t2435 * t7015;
    let t25372 = t786 * t251;
    let t25373 = t1032 * t2769;
    let t25374 = t25373 * t233;
    let t25375 = t25372 * t25374;
    let t25377 = t1949 * t72 * t122;
    let t25378 = t25377 * t2466;
    let t25379 = t25375 * t25378;
    let t25383 = t1955 * t25308;
    (t25366, t25368, t25371, t25372, t25373, t25374, t25375, t25377, t25378, t25379, t25383)
}
