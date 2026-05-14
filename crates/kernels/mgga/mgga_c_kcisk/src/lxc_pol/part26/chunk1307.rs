//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1307/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1307<F: Float>(t113620: F, t113622: F, t113832: F, t114664: F, t118718: F, t118741: F, t118745: F, t118748: F, t118754: F, t118759: F, t118766: F, t13504: F, t25342: F, t32087: F, t32096: F, t33360: F, t33410: F, t33415: F, t34744: F, t6175: F, t9426: F, t9449: F) -> (F,) {
    let t118777 = -0.69444444444444444446e-2 * t32087 * t118741 + 0.44218518518518518516e-2 * t118745 + 0.11054629629629629629e-2 * t118748 - 0.69444444444444444447e-2 * t118718 * t9449 + 0.44229166666666666667e-1 * t9426 * t118754 - t113620 - t113622 - 0.20833333333333333334e-1 * t32096 * t34744 + 0.46296296296296296297e-2 * t32087 * t13504 * t33415 * t118759 + 0.46296296296296296297e-2 * t32087 * t118766 + 0.27777777777777777778e-1 * t32087 * t6175 * t113832 * t25342 + 0.69444444444444444446e-2 * t114664 * t33360 + 0.13888888888888888889e-1 * t114664 * t33410;
    (t118777,)
}
