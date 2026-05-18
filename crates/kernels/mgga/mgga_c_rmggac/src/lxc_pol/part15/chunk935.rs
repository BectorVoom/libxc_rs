//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 935/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk935<F: Float>(t45343: F, t674: F, t2007: F, t321: F, t9888: F, t262: F, t36629: F, t333: F, t41634: F, t39507: F, t45514: F, t45519: F, t45523: F, t45525: F, t45527: F, t45531: F, t45536: F, t45541: F, t45546: F, t45550: F, t45554: F, t45559: F, t530: F, t623: F, t739: F, t8795: F) -> (F, F, F, F, F, F) {
    let t45561 = t45343 * t674;
    let t45562 = t45561 * t2007;
    let t45568 = t9888 * t321;
    let t45569 = t262 * t45568;
    let t45570 = t36629 * t45569;
    let t45572 = t9888 * t333;
    let t45573 = t262 * t45572;
    let t45574 = t41634 * t45573;
    let t45576 = -F::new(0.1064114997332445985e-4) * t45514 + F::new(0.85129199786595678796e-5) * t45519 - F::new(0.53205749866622299248e-5) * t45523 - F::new(0.25538759935978703638e-4) * t45525 - F::new(0.11974241701863808564e0) * t739 * t45527 + F::new(0.34093327067806677161e-2) * t45531 + F::new(0.1064114997332445985e-4) * t45536 - F::new(0.1064114997332445985e-4) * t45541 - F::new(0.85129199786595678796e-5) * t45546 + F::new(0.25538759935978703639e-4) * t45550 - F::new(0.25538759935978703639e-4) * t45554 - F::new(0.25538759935978703639e-4) * t45559 + F::new(0.25538759935978703639e-4) * t45562 - F::new(0.39914139006212695214e-1) * t623 * t8795 - F::new(0.4726e1) * t530 * t39507 + F::new(0.20455996240684006296e0) * t45570 - F::new(0.40911992481368012592e0) * t45574;
    (t45561, t45568, t45569, t45572, t45573, t45576)
}
