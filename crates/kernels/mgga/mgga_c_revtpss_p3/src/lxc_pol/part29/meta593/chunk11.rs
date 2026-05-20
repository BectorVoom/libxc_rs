//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1988/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1988<F: Float>(t1385: F, t8085: F, t1903: F, t26304: F, t25930: F, t25933: F, t27864: F, t27868: F, t27972: F, t28911: F, t28915: F, t48025: F, t94705: F, t94823: F, t96392: F, t96549: F, t96550: F, t96552: F, t96556: F, t96559: F, t96561: F, t96564: F, t96565: F) -> F {
    let t102656 = t1385 * t8085;
    let t102661 = t26304 * t1903;
    let t102669 = -F::cast_from(0.17347256376410398924e1_f64) * t94705 * t28915 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t96392 * t27972 + t96549 + F::cast_from(0.72280234901709995518e-2_f64) * t96550 - F::cast_from(0.8673628188205199462e0_f64) * t27868 * t28911 * t48025 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t96392 * t27864 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t102656 * t25933 + F::cast_from(0.51405703062096148812e-1_f64) * t96552 + F::cast_from(0.52041769129231196772e1_f64) * t94823 * t102661 * t25933 + F::cast_from(0.54878743191129263322e-2_f64) * t96556 + F::cast_from(0.13009920719177044025e-2_f64) * t96559 - F::cast_from(0.2601984143835408805e-1_f64) * t96561 - t96564 + F::cast_from(0.38549458614245330943e-1_f64) * t96565;
    t102669
}
