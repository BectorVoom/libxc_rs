//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1356/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1356<F: Float>(t109541: F, t110219: F, t110325: F, t110335: F, t110365: F, t110384: F, t110423: F, t110524: F, t113779: F, t113783: F, t113788: F, t113792: F, t113796: F, t113800: F, t113805: F, t32102: F, t33360: F, t33424: F, t9446: F) -> (F,) {
    let t113811 = -0.69444444444444444446e-2 * t110325 - 0.11574074074074074074e-2 * t110335 - 0.22109259259259259258e-2 * t109541 - 0.3684876543209876543e-3 * t113779 - 0.20833333333333333334e-1 * t9446 * t113783 - 0.46561250000000000002e-2 * t32102 * t113788 - 0.22109259259259259258e-2 * t113792 + 0.61728395061728395063e-2 * t110365 + 0.33163888888888888888e-2 * t113796 - 0.71481481481481481483e-2 * t110219 * t33360 + 0.89351851851851851853e-3 * t113800 - 0.18518518518518518519e-1 * t110524 * t33424 + 0.23148148148148148148e-2 * t113805 + 0.69444444444444444446e-2 * t110423 * t33360 + 0.69444444444444444446e-2 * t110384 * t33360;
    (t113811,)
}
