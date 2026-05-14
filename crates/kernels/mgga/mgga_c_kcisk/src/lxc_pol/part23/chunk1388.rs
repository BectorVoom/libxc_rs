//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1388/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1388<F: Float>(t114608: F, t1339: F, t9462: F, t394: F, t5885: F, t3799: F, t32058: F, t415: F, t5968: F, t3805: F, t9821: F, t110347: F, t110351: F, t114075: F, t114588: F, t114592: F, t114597: F, t114604: F, t114606: F, t32030: F, t32035: F, t33373: F, t33384: F, t9429: F, t9454: F, t9805: F) -> (F, F, F, F, F, F) {
    let t114610 = t1339 * t114608 * t9462;
    let t114618 = t5885 * t394;
    let t114620 = t1339 * t114618 * t3799;
    let t114623 = t415 * t32058 * t5968;
    let t114625 = t3805 * t9821;
    let t114627 = -0.1492375e-1 * t114588 - 0.2653111111111111111e-1 * t114592 + 0.10416666666666666667e-1 * t33373 * t32030 - t114597 + 0.20833333333333333334e-1 * t114075 * t9454 + 0.20833333333333333334e-1 * t114075 * t9429 + 0.73697530864197530862e-3 * t114604 + 0.22109259259259259258e-2 * t114606 - 0.88437037037037037034e-2 * t114610 - 0.20833333333333333334e-1 * t33384 * t32035 - 0.34722222222222222223e-2 * t110347 * t9805 - 0.69444444444444444446e-2 * t110351 * t9805 - 0.33163888888888888888e-2 * t114620 - 0.49745833333333333332e-2 * t114623 + 0.55273148148148148147e-3 * t114625;
    (t114610, t114618, t114620, t114623, t114625, t114627)
}
