//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1083/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1083<F: Float>(t2097: F, t3722: F, t1212: F, t19691: F, t1528: F, t6515: F, t2285: F, t4460: F, t19483: F, t12929: F, t19134: F, t19138: F, t19142: F, t19212: F, t19214: F, t19510: F, t19513: F, t19516: F, t19519: F, t19524: F) -> (F, F, F, F, F, F) {
    let t21748 = t2097 * t3722;
    let t21755 = t19691 * t1212;
    let t21759 = t6515 * t1528;
    let t21764 = t2285 * t4460;
    let t21771 = 0.27785333333333333334e0 * t19483;
    let t21793 = 0.41318e1 * t19134 + 0.103295e1 * t19138 - 0.103295e1 * t19142 - 0.45908888888888888888e0 * t12929 - 0.20839e0 * t19510 - 0.13892666666666666667e0 * t19513 + 0.20839e0 * t19516 + 0.83356e0 * t19519 - 0.3529725e1 * t19212 - 0.17648625e1 * t19214 + 0.6311625e0 * t19524;
    (t21748, t21755, t21759, t21764, t21771, t21793)
}
