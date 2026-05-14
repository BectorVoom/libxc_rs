//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1216/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1216<F: Float>(t1849: F, t2029: F, t6667: F, t33225: F, t10005: F, t10014: F, t32901: F, t32925: F, t33235: F, t34065: F, t34081: F, t34087: F, t34091: F, t34496: F, t9721: F, t9733: F, t9740: F, t9748: F) -> (F, F, F, F) {
    let t34499 = t2029 * t1849;
    let t34500 = t34499 * t6667;
    let t34501 = t33225 * t34500;
    let t34511 = 0.77382407407407407407e-3 * t32901 + 0.52083333333333333333e-2 * t9721 * t10014 + 0.52083333333333333333e-2 * t9733 * t10014 + 0.77382407407407407407e-3 * t34065 + 0.17361111111111111111e-2 * t9740 * t34496 + 0.34722222222222222222e-2 * t9740 * t34501 + 0.17361111111111111111e-2 * t33235 + 0.77382407407407407407e-3 * t32925 - 0.13888888888888888889e-1 * t10005 * t9748 - 0.11607361111111111111e-2 * t34081 - 0.11607361111111111111e-2 * t34087 - 0.38691203703703703703e-3 * t34091;
    (t34499, t34500, t34501, t34511)
}
