//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1320/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1320<F: Float>(t1339: F, t32000: F, t34809: F, t113955: F, t113963: F, t114001: F, t114004: F, t114121: F, t119066: F, t119069: F, t119072: F, t119076: F, t119079: F, t119083: F, t119088: F, t32022: F, t34707: F, t9796: F) -> (F, F) {
    let t119091 = t1339 * t32000 * t34809;
    let t119093 = -t113955 + 0.33163888888888888888e-2 * t119066 + 0.66327777777777777776e-2 * t119069 + 0.16581944444444444444e-2 * t119072 + 0.16581944444444444444e-2 * t119076 + 0.27636574074074074073e-2 * t119079 - 0.27777777777777777779e-1 * t32022 * t34707 - 0.22109259259259259259e-2 * t119083 + 0.23148148148148148149e-2 * t113963 - 0.21444444444444444445e-1 * t114121 * t9796 - 0.22109259259259259259e-2 * t119088 + 0.88437037037037037035e-2 * t119091 + t114001 + t114004;
    (t119091, t119093)
}
