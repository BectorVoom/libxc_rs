//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1006/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1006<F: Float>(t39437: F, t10710: F, t25499: F, t37586: F, t25503: F, t37658: F, t11816: F, t37880: F, t3308: F, t6449: F, t7462: F, t39421: F, t39422: F, t39424: F, t39426: F, t39429: F, t39431: F, t39434: F) -> (F,) {
    let t39438 = 0.47609969197673950972e-2 * t39437;
    let t39440 = t37586 * t10710 * t25499;
    let t39443 = t37658 * t10710 * t25503;
    let t39444 = 0.14282990759302185292e-1 * t39443;
    let t39445 = t37880 * t11816;
    let t39446 = 0.47609969197673950972e-2 * t39445;
    let t39448 = t6449 * t3308 * t7462;
    let t39450 = -t39421 + 0.54878743191129263322e-1 * t39422 + 0.10975748638225852664e0 * t39424 - 0.27439371595564631661e-1 * t39426 + 0.15573871527278325618e-1 * t39429 + 0.54878743191129263322e-1 * t39431 + 0.86682217400542685632e-1 * t39434 + t39438 - 0.95219938395347901943e-2 * t39440 - t39444 + t39446 - 0.10401866088065122276e1 * t39448;
    (t39450,)
}
