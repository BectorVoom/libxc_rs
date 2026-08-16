//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1267/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1267<F: Float>(t94669: F, t94671: F, t25894: F, t94668: F, t25950: F, t25953: F, t26069: F, t94407: F, t1426: F, t9990: F, t1444: F, t2022: F, t2030: F, t25921: F, t26034: F, t26046: F, t26079: F, t4003: F, t7295: F, t7296: F, t94413: F, t94641: F, t94643: F, t94648: F, t94650: F, t94656: F, t94662: F, t94665: F, t9658: F, t9994: F) -> F {
    let t94672 = t94669 * t94671;
    let t94674 = t25894 * t94668;
    let t94675 = t94674 * t94671;
    let t94677 = t25950 * t25953;
    let t94682 = F::cast_from(0.91399340044406952588e-2_f64) * t26069 * t94407;
    let t94683 = t1426 * t9990;
    let t94692 = F::cast_from(0.38554277296572111609e-1_f64) * t94641 - F::cast_from(0.4336814094102599731e0_f64) * t94643 * t2030 + t94648 - F::cast_from(0.15421710918628844643e0_f64) * t94650 + F::cast_from(0.26020884564615598386e1_f64) * t7295 * t7296 * t26034 * t1444 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t2022 * t9658 + F::cast_from(0.57824187921367996415e-1_f64) * t94662 - F::cast_from(0.43368140941025997312e-1_f64) * t94665 - F::cast_from(0.23132566377943266966e0_f64) * t94672 + F::cast_from(0.13010442282307799194e0_f64) * t94675 + F::cast_from(0.51405703062096148812e-1_f64) * t94677 + F::cast_from(0.13010442282307799193e1_f64) * t25921 * t26046 + t94682 + F::cast_from(0.26020884564615598386e1_f64) * t7295 * t94683 * t94413 * t9994 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t26079 * t94413 * t4003;
    t94692
}
