//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3727/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3727<F: Float>(t12916: F, t20837: F, t5331: F, t12910: F, t21003: F, t5245: F, t5284: F, t1214: F, t20836: F, t1250: F, t12787: F, t12809: F, t17605: F, t17623: F, t17669: F, t17674: F, t17677: F, t17682: F, t17703: F, t17747: F, t20956: F, t21040: F, t3718: F, t3720: F, t5332: F, t5340: F, t57435: F, t57449: F, t57451: F, t6421: F) -> (F, F, F) {
    let t70685 = t5331 * t12916 * t20837;
    let t70689 = t12910 * t12916 * t21003;
    let t70693 = t5245 * t5284;
    let t70712 = t20836 * t1214;
    let t70717 = F::cast_from(0.30488190661738479624e-2_f64) * t17605 * t17669 + F::cast_from(0.42874018118069736972e-3_f64) * t12910 * t3720 * t21040 * t17623 - F::cast_from(0.57165357490759649296e-3_f64) * t70685 + F::cast_from(0.28582678745379824648e-3_f64) * t57435 + F::cast_from(0.11433071498151929859e-2_f64) * t70689 - F::cast_from(0.19055119163586549765e-3_f64) * t57449 - F::cast_from(0.3811023832717309953e-3_f64) * t57451 - F::cast_from(0.85748036236139473944e-3_f64) * t3718 * t3720 * t70693 * t1250 - F::cast_from(0.12862205435420921092e-2_f64) * t17747 * t3720 * t20956 * t17703 + F::cast_from(0.47637797908966374414e-3_f64) * t5340 * t12787 * t6421 * t17677 - F::cast_from(0.23818898954483187207e-3_f64) * t5331 * t12787 * t6421 * t17682 + F::cast_from(0.15244095330869239812e-2_f64) * t17605 * t17674 + F::cast_from(0.85748036236139473944e-3_f64) * t12809 * t3720 * t5332 * t70712;
    (t70693, t70712, t70717)
}
