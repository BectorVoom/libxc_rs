//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2216/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2216<F: Float>(t108530: F, t108551: F, t108564: F, t108580: F, t108589: F, t108596: F, t108613: F, t108631: F, t108502: F, t14230: F, t1903: F, t213: F, t22395: F, t225: F, t25930: F, t25931: F, t27868: F, t27980: F, t561: F, t7279: F, t75016: F, t94884: F, t98333: F, t98338: F, t98358: F, t98360: F, t98368: F, t98372: F, t98376: F, t98379: F) -> (F, F) {
    let t108634 = t108530 + t108551 + t108564 + t108580 + t108589 + t108596 + t108613 + t108631;
    let t108651 = F::cast_from(0.17347256376410398924e1_f64) * t25930 * t27980 * t108502 + F::cast_from(0.13009920719177044025e-1_f64) * t94884 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t108634 * t225 * t561 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t25931 * t75016 - F::cast_from(0.68540937416128198416e-1_f64) * t98333 - F::cast_from(0.68540937416128198419e-2_f64) * t98338 - t98358 - t98360 + F::cast_from(0.26341796731742046394e1_f64) * t7279 * t22395 + F::cast_from(0.34694512752820797848e1_f64) * t25930 * t27980 * t1903 * t14230 - t98368 - F::cast_from(0.26019841438354088051e-1_f64) * t98372 + t98376 + t98379;
    (t108634, t108651)
}
