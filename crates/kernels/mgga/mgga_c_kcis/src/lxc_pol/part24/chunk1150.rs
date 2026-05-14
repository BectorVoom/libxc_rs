//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1150/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1150<F: Float>(t101355: F, t7690: F, t1704: F, t1709: F, t922: F, t93426: F, t100596: F, t100599: F, t100602: F, t101195: F, t28997: F, t7687: F, t8034: F, t92993: F, t92997: F, t93425: F, t93592: F, t96026: F, t96522: F) -> (F, F) {
    let t101372 = t7690 * t101355;
    let t101374 = t1704 * t1709;
    let t101376 = t93426 * t101374 * t922;
    let t101383 = 0.16581944444444444444e-2 * t100596 + 0.16581944444444444444e-2 * t100599 + 0.33163888888888888888e-2 * t100602 - 0.13901041666666666667e-2 * t7687 * t28997 + 0.18550940104166666667e-3 * t96522 * t8034 - 0.92754700520833333333e-4 * t101372 - 0.46336805555555555557e-3 * t93592 * t101376 + 0.82448622685185185185e-4 * t93425 * t101195 + t96026 + 0.55273148148148148147e-3 * t92993 - 0.36848765432098765431e-3 * t92997;
    (t101376, t101383)
}
