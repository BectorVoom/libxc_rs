//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1385/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1385<F: Float>(t13546: F, t977: F, t13555: F, t2979: F, t13528: F, t13532: F, t10214: F, t13537: F, t13969: F, t4595: F, t3130: F, t1616: F, t2780: F) -> (F, F, F, F, F, F, F) {
    let t14006 = t977 * t13546;
    let t14009 = t2979 * t13555;
    let t14012 = t2979 * t13528;
    let t14015 = t2979 * t13532;
    let t14018 = t10214 * t13537;
    let t14025 = t13969 * t4595;
    let t14027 = t3130 * t14025 / F::cast_from(1152.0_f64);
    let t14032 = t1616 * t2780;
    (t14006, t14009, t14012, t14015, t14018, t14027, t14032)
}
