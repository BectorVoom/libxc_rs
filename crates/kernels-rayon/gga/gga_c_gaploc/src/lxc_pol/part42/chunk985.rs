//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 985/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk985(t49989: f64, t688: f64, t123: f64, t14435: f64, t1841: f64, t2508: f64, t42985: f64, t42988: f64, t42991: f64, t44786: f64, t44790: f64, t44792: f64, t44798: f64, t44802: f64, t44805: f64, t44809: f64, t44812: f64, t44818: f64, t44820: f64, t44823: f64, t44826: f64, t50122: f64, t734: f64, t779: f64) -> f64 {
    let t50366 = t49989 * t688;
    let t50373 = 0.25635087433807414279e-2_f64 * t42985 + 0.25635087433807414279e-2_f64 * t42988 + 0.25635087433807414279e-2_f64 * t42991 - t44786 + t44790 + 0.96131577876777803547e-3_f64 * t44792 - t44798 + t44802 - t44805 - 0.85450291446024714263e-3_f64 * t1841 * t50122 * t123 * t734 - t44809 - t44812 + t44818 - t44820 + 0.15381052460284448567e-1_f64 * t2508 * t779 * t50366 + 0.76905262301422242837e-2_f64 * t2508 * t779 * t14435 - t44823 + t44826;
    t50373
}
