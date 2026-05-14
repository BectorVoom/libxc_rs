//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1210/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1210<F: Float>(t33451: F, t9446: F, t32008: F, t32087: F, t32107: F, t32108: F, t32115: F, t33373: F, t33424: F, t33428: F, t33434: F, t33439: F, t33446: F, t9426: F, t9454: F, t2158: F, t3930: F) -> (F, F) {
    let t33452 = t9446 * t33451;
    let t33454 = 0.34722222222222222223e-2 * t32087 * t33424 + 0.13402777777777777778e-2 * t32008 * t33428 + 0.10416666666666666667e-1 * t33373 * t9454 - 0.20833333333333333334e-1 * t9446 * t33434 - 0.10416666666666666667e-1 * t9446 * t33439 - 0.40208333333333333335e-2 * t9426 * t33439 + 0.34722222222222222223e-2 * t32087 * t33446 + t32107 - 0.34722222222222222223e-2 * t32108 + 0.92592592592592592595e-2 * t32115 + 0.34722222222222222223e-2 * t33452;
    let t33459 = t3930 * t2158;
    (t33454, t33459)
}
