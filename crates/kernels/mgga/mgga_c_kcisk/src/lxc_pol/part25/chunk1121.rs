//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1121/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1121<F: Float>(t1908: F, t33130: F, t2049: F, t9772: F, t33089: F, t33092: F, t33095: F, t33099: F, t33101: F, t33104: F, t33107: F, t33110: F, t33112: F, t33114: F, t33116: F, t33118: F, t33122: F, t33124: F, t33126: F, t33128: F) -> (F, F, F) {
    let t33131 = t1908 * t33130;
    let t33132 = t9772 * t2049;
    let t33151 = 0.9375e-1 * t33089 - 0.1875e0 * t33092 + 0.125e0 * t33095 + 0.1875e0 * t33099 - 0.125e0 * t33101 - 0.9375e-1 * t33104 - 0.20833333333333333333e-1 * t33107 + 0.625e-1 * t33110 - 0.20234375e-1 * t33112 + 0.4046875e-1 * t33114 - 0.53958333333333333334e-1 * t33116 - 0.4046875e-1 * t33118 + 0.53958333333333333334e-1 * t33122 + 0.20234375e-1 * t33124 - 0.89930555555555555557e-2 * t33126 - 0.26979166666666666667e-1 * t33128;
    (t33131, t33132, t33151)
}
