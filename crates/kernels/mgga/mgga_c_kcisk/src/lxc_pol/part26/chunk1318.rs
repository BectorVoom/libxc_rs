//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1318/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1318<F: Float>(t110384: F, t110423: F, t113920: F, t113923: F, t113933: F, t113939: F, t113941: F, t113951: F, t20053: F, t20067: F, t2059: F, t25413: F, t32087: F, t32088: F, t33408: F, t33428: F, t34763: F, t34768: F, t3937: F, t6211: F, t88000: F) -> (F,) {
    let t119064 = -0.44218518518518518516e-2 * t113920 + t113923 - t113933 + 0.15432098765432098765e-2 * t113939 + 0.69444444444444444446e-2 * t110423 * t34768 + 0.69444444444444444446e-2 * t110384 * t34768 + 0.69444444444444444446e-2 * t32087 * t3937 * t32088 * t2059 * t6211 + 0.13888888888888888889e-1 * t32087 * t20067 * t32088 * t88000 + 0.69444444444444444446e-2 * t110423 * t34763 + 0.69444444444444444446e-2 * t110384 * t34763 + 0.27777777777777777778e-1 * t32087 * t20053 * t33408 * t25413 + 0.69444444444444444447e-2 * t113941 * t33428 + t113951;
    (t119064,)
}
