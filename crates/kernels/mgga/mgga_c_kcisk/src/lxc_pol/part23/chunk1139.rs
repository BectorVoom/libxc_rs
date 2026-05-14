//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1139/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1139<F: Float>(t1220: F, t32142: F, t2722: F, t3907: F, t415: F, t1333: F, t9466: F, t9470: F, t9478: F, t13448: F, t2714: F, t1292: F, t1299: F, t20: F, t2718: F, t32030: F, t32035: F, t32072: F, t32096: F, t32120: F, t32124: F, t32127: F, t32131: F, t32139: F, t9426: F, t9446: F, t9449: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32143 = t1220 * t32142;
    let t32150 = t3907 * t2722;
    let t32151 = t415 * t32150;
    let t32153 = t1333 * t9466;
    let t32155 = t1333 * t9470;
    let t32157 = t1333 * t9478;
    let t32159 = t13448 * t2714;
    let t32163 = t1292 * t1299 * t20;
    let t32164 = t1220 * t32163;
    let t32167 = -0.55273148148148148147e-3 * t32120 + 0.49745833333333333332e-2 * t32124 - 0.34722222222222222223e-2 * t9446 * t32127 - 0.46296296296296296297e-2 * t9446 * t32131 - 0.20833333333333333334e-1 * t9446 * t32072 - 0.69444444444444444446e-2 * t32096 * t9449 - 0.24872916666666666666e-2 * t32139 - 0.10416666666666666667e-1 * t32143 * t2718 + 0.40208333333333333335e-2 * t9426 * t32030 - 0.120625e-1 * t9426 * t32035 + 0.24872916666666666666e-2 * t32151 + 0.33163888888888888888e-2 * t32153 - 0.33163888888888888888e-2 * t32155 + 0.22109259259259259258e-2 * t32157 - 0.10416666666666666667e-1 * t32159 * t2718 + 0.55555555555555555558e-1 * t32164 * t2718;
    (t32143, t32150, t32151, t32153, t32155, t32157, t32159, t32163, t32164, t32167)
}
