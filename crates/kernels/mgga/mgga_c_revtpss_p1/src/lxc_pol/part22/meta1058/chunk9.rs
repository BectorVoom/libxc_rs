//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3762/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3762<F: Float>(t20851: F, t3678: F, t17290: F, t5362: F, t17435: F, t5327: F, t3655: F, t6595: F, t1256: F, t21313: F, t21316: F, t17332: F, t17638: F, t17644: F, t17672: F, t17677: F, t17682: F, t1803: F, t20272: F, t20795: F, t3625: F, t3626: F, t484: F, t5331: F, t5340: F, t5405: F, t6425: F, t6429: F) -> F {
    let t71738 = t20851 * t3678;
    let t71740 = t17290 * t5362;
    let t71742 = t5327 * t17435;
    let t71744 = t6595 * t3655;
    let t71749 = t21313 * t1256;
    let t71751 = t21316 * t1256;
    let t71781 = -F::cast_from(0.28582678745379824648e-3_f64) * t71738 - F::cast_from(0.57165357490759649296e-3_f64) * t71740 - F::cast_from(0.57165357490759649296e-3_f64) * t71742 - F::cast_from(0.16090989515917530913e-2_f64) * t71744 - F::cast_from(0.22866142996303859718e-2_f64) * t17332 * t1803 * t484 + F::cast_from(0.96545937095505185476e-2_f64) * t71749 - F::cast_from(0.30488190661738479624e-2_f64) * t71751 - F::cast_from(0.28582678745379824648e-3_f64) * t5340 * t3626 * t6429 * t17677 + F::cast_from(0.14291339372689912324e-3_f64) * t5331 * t3626 * t6429 * t17682 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t3626 * t6425 * t17672 - F::cast_from(0.57165357490759649296e-3_f64) * t5340 * t3626 * t6425 * t17677 + F::cast_from(0.14291339372689912324e-3_f64) * t5331 * t3626 * t20795 * t17638 + F::cast_from(0.28582678745379824648e-3_f64) * t5331 * t3626 * t20795 * t17644 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t3626 * t20272 * t5405;
    t71781
}
