//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1412/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1412<F: Float>(t33921: F, t964: F, t25: F, t33924: F, t9536: F, t109499: F, t109518: F, t109664: F, t113761: F, t115139: F, t19087: F, t21492: F, t21652: F, t2740: F, t32339: F, t32354: F, t32362: F, t32436: F, t32439: F, t32458: F, t32459: F, t33771: F, t33807: F, t33817: F, t33911: F, t33916: F, t33923: F, t33925: F, t9528: F, t9859: F) -> (F,) {
    let t115169 = t964 * t33921;
    let t115179 = 0.15432098765432098765e-2 * t9536 * t25 * t33921 * t33924;
    let t115209 = -0.92592592592592592592e-2 * t9536 * t115169 * t33923 * t19087 - 0.92592592592592592593e-2 * t32339 * t33817 - t115179 + 0.17361111111111111111e-2 * t9536 * t32458 * t32459 * t21492 + 0.34722222222222222222e-2 * t32436 * t33771 - 0.34722222222222222222e-2 * t9536 * t109499 * t32459 * t21652 + 0.13402777777777777778e-2 * t32439 * t115139 + 0.34722222222222222222e-2 * t32436 * t33911 + 0.69444444444444444444e-2 * t32436 * t33916 + 0.13402777777777777778e-2 * t109664 * t33911 + 0.13402777777777777778e-2 * t109518 * t33911 - 0.46296296296296296296e-2 * t32354 * t33925 + 0.11349419753086419753e-1 * t113761 + 0.27777777777777777778e-1 * t33807 * t9528 * t2740 - 0.52083333333333333333e-2 * t32362 * t9859 * t2740;
    (t115209,)
}
