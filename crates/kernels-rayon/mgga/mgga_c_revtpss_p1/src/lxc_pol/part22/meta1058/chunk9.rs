//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3762/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3762(t20851: f64, t3678: f64, t17290: f64, t5362: f64, t17435: f64, t5327: f64, t3655: f64, t6595: f64, t1256: f64, t21313: f64, t21316: f64, t17332: f64, t17638: f64, t17644: f64, t17672: f64, t17677: f64, t17682: f64, t1803: f64, t20272: f64, t20795: f64, t3625: f64, t3626: f64, t484: f64, t5331: f64, t5340: f64, t5405: f64, t6425: f64, t6429: f64) -> f64 {
    let t71738 = t20851 * t3678;
    let t71740 = t17290 * t5362;
    let t71742 = t5327 * t17435;
    let t71744 = t6595 * t3655;
    let t71749 = t21313 * t1256;
    let t71751 = t21316 * t1256;
    let t71781 = -0.28582678745379824648e-3_f64 * t71738 - 0.57165357490759649296e-3_f64 * t71740 - 0.57165357490759649296e-3_f64 * t71742 - 0.16090989515917530913e-2_f64 * t71744 - 0.22866142996303859718e-2_f64 * t17332 * t1803 * t484 + 0.96545937095505185476e-2_f64 * t71749 - 0.30488190661738479624e-2_f64 * t71751 - 0.28582678745379824648e-3_f64 * t5340 * t3626 * t6429 * t17677 + 0.14291339372689912324e-3_f64 * t5331 * t3626 * t6429 * t17682 - 0.28582678745379824648e-3_f64 * t3625 * t3626 * t6425 * t17672 - 0.57165357490759649296e-3_f64 * t5340 * t3626 * t6425 * t17677 + 0.14291339372689912324e-3_f64 * t5331 * t3626 * t20795 * t17638 + 0.28582678745379824648e-3_f64 * t5331 * t3626 * t20795 * t17644 - 0.28582678745379824648e-3_f64 * t3625 * t3626 * t20272 * t5405;
    t71781
}
