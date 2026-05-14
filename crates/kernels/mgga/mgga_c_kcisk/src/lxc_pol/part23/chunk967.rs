//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 967/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk967<F: Float>(t19642: F, t321: F, t19483: F, t12929: F, t19134: F, t19138: F, t19142: F, t19212: F, t19214: F, t19510: F, t19513: F, t19516: F, t19519: F, t19524: F, t19102: F, t12967: F, t13091: F, t13092: F, t19104: F, t19111: F, t19192: F, t19207: F, t19543: F, t19545: F, t19549: F, t19551: F) -> (F, F, F, F, F) {
    let t19644 = 0.62182e-1 * t19642 * t321;
    let t19645 = 0.22076e0 * t19483;
    let t19667 = 0.24154e1 * t19134 + 0.60385e0 * t19138 - 0.60385e0 * t19142 - 0.26837777777777777778e0 * t12929 - 0.16557e0 * t19510 - 0.11038e0 * t19513 + 0.16557e0 * t19516 + 0.66228e0 * t19519 - 0.258925e1 * t19212 - 0.1294625e1 * t19214 + 0.16504875e0 * t19524;
    let t19678 = 0.13418888888888888889e0 * t19102;
    let t19689 = -0.40256666666666666667e0 * t19104 - 0.33547222222222222222e0 * t19111 - 0.11038e0 * t12967 - t13091 - t13092 - 0.91983333333333333334e-1 * t19543 + 0.71747e0 * t19545 + 0.19419375e1 * t19207 - 0.412621875e-1 * t19549 + 0.16504875e0 * t19551 + 0.258925e1 * t19192;
    (t19644, t19645, t19667, t19678, t19689)
}
