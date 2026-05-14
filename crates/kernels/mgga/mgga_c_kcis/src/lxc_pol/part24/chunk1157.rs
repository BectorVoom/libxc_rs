//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1157/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1157<F: Float>(t1250: F, t71731: F, t27974: F, t8030: F, t101028: F, t101208: F, t27958: F, t2894: F, t71387: F, t7693: F, t7703: F, t7704: F, t95524: F, t96412: F, t96418: F, t96428: F, t96449: F, t96451: F) -> (F,) {
    let t101554 = t71731 * t1250;
    let t101567 = t8030 * t27974;
    let t101569 = -0.7369753086419753086e-3 * t96412 + 0.92754700520833333333e-4 * t101554 * t7693 + 0.6183646701388888889e-4 * t95524 * t27958 - 0.46336805555555555557e-3 * t7703 * t101208 + 0.23168402777777777778e-3 * t7703 * t2894 * t7704 * t71387 + 0.23168402777777777778e-3 * t7703 * t101028 + 0.46336805555555555557e-3 * t101567 - t96418 + t96428 + t96449 - t96451;
    (t101569,)
}
