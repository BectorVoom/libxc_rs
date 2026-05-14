//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1188/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1188<F: Float>(t32013: F, t8048: F, t6204: F, t32102: F, t33373: F, t33470: F, t33485: F, t33501: F, t33530: F, t34693: F, t34697: F, t34725: F, t34728: F, t34737: F, t34740: F, t34744: F, t9426: F, t9446: F, t9809: F) -> (F, F, F) {
    let t34748 = t32013 * t8048;
    let t34749 = t6204 * t34748;
    let t34755 = -0.33163888888888888888e-2 * t34725 + 0.33163888888888888888e-2 * t34728 - 0.23148148148148148148e-2 * t33470 + 0.40208333333333333335e-2 * t9426 * t34697 + 0.20833333333333333334e-1 * t33373 * t9809 - 0.69444444444444444446e-2 * t33485 + 0.24872916666666666666e-2 * t34737 + 0.22109259259259259258e-2 * t34740 - 0.20833333333333333334e-1 * t9446 * t34744 + 0.22109259259259259258e-2 * t33501 - 0.20833333333333333334e-1 * t9446 * t34749 - 0.23280625000000000001e-2 * t32102 * t34693 + 0.22109259259259259258e-2 * t33530;
    (t34748, t34749, t34755)
}
