//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 985/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk985<F: Float>(t49989: F, t688: F, t123: F, t14435: F, t1841: F, t2508: F, t42985: F, t42988: F, t42991: F, t44786: F, t44790: F, t44792: F, t44798: F, t44802: F, t44805: F, t44809: F, t44812: F, t44818: F, t44820: F, t44823: F, t44826: F, t50122: F, t734: F, t779: F) -> F {
    let t50366 = t49989 * t688;
    let t50373 = F::cast_from(0.25635087433807414279e-2_f64) * t42985 + F::cast_from(0.25635087433807414279e-2_f64) * t42988 + F::cast_from(0.25635087433807414279e-2_f64) * t42991 - t44786 + t44790 + F::cast_from(0.96131577876777803547e-3_f64) * t44792 - t44798 + t44802 - t44805 - F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t50122 * t123 * t734 - t44809 - t44812 + t44818 - t44820 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t779 * t50366 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t779 * t14435 - t44823 + t44826;
    t50373
}
