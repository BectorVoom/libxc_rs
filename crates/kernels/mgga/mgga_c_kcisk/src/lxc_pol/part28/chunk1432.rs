//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1432/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1432<F: Float>(t10886: F, t35388: F, t9740: F, t17182: F, t35401: F, t33196: F, t34389: F, t34435: F, t2063: F, t33225: F, t33226: F, t7644: F, t117934: F, t220: F, t2647: F, t10009: F, t112876: F, t117702: F, t117784: F, t118460: F, t121691: F, t1636: F, t2023: F, t33208: F, t33297: F, t34534: F, t35389: F, t35444: F, t35454: F, t7261: F, t9176: F) -> (F, F, F, F) {
    let t122938 = t9740 * t10886 * t35388;
    let t122940 = t17182 * t35401;
    let t122941 = t33196 * t122940;
    let t122943 = t34435 * t34389;
    let t122965 = t33225 * t33226 * t2063 * t7644;
    let t122970 = t117934 * t33226 * t220 * t2647;
    let t122973 = 0.11607361111111111111e-2 * t121691 + 0.31250000000000000001e-1 * t9740 * t7261 * t118460 * t9176 * t2023 - 0.77160493827160493827e-3 * t122938 - 0.13402777777777777778e-2 * t122941 - 0.11574074074074074074e-2 * t122943 - 0.23148148148148148148e-2 * t33297 * t35389 - 0.23148148148148148148e-2 * t33208 * t35389 + 0.92592592592592592593e-2 * t117702 * t10009 + 0.92592592592592592593e-2 * t117784 * t10009 + 0.34722222222222222223e-2 * t34435 * t34534 - 0.34722222222222222223e-2 * t9740 * t112876 * t35444 * t1636 + 0.34722222222222222222e-2 * t33297 * t35454 + 0.34722222222222222222e-2 * t33208 * t35454 + 0.34722222222222222222e-2 * t9740 * t122965 - 0.69444444444444444444e-2 * t9740 * t122970;
    (t122940, t122965, t122970, t122973)
}
