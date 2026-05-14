//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 771/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk771<F: Float>(t13616: F, t1526: F, t15567: F, t17685: F, t17688: F, t17695: F, t17698: F, t17703: F, t17708: F, t2320: F, t342: F, t343: F, t3683: F, t3695: F, t3713: F, t3827: F, t9482: F, t9485: F, t9488: F) -> (F,) {
    let t17712 = t3683 + t3827 + t9482 - t9485 / 36.0 - t9488 / 12.0 - t17685 / 36.0 - t15567 * t17688 / 9.0 - t1526 * t2320 * t3695 / 12.0 + t15567 * t17695 / 6.0 + t1526 * t13616 * t17698 / 6.0 - t17703 / 12.0 - t1526 * t2320 * t3713 / 12.0 - t342 * t343 * t17708 / 4.0;
    (t17712,)
}
