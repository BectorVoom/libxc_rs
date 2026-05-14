//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1241/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1241<F: Float>(t33873: F, t9516: F, t2737: F, t2740: F, t32423: F, t32425: F, t32439: F, t33771: F, t33837: F, t33851: F, t33854: F, t33864: F, t33871: F, t9512: F, t9524: F, t9529: F, t9536: F, t9869: F) -> (F,) {
    let t33874 = t9516 * t33873;
    let t33880 = 0.13888888888888888889e-1 * t33851 * t2740 - 0.52083333333333333333e-2 * t33854 * t2740 + 0.52083333333333333333e-2 * t9512 * t9869 + 0.52083333333333333333e-2 * t9524 * t9869 + 0.17361111111111111111e-2 * t32423 + 0.52083333333333333333e-2 * t2737 * t33864 + 0.17361111111111111111e-2 * t32425 - 0.13888888888888888889e-1 * t9529 * t9869 + 0.17361111111111111111e-2 * t33871 + 0.6701388888888888889e-3 * t33874 + 0.17361111111111111111e-2 * t9536 * t33771 - 0.20104166666666666667e-2 * t32439 * t33837;
    (t33880,)
}
