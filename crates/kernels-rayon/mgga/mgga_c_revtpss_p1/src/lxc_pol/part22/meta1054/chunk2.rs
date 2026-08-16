//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3727/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3727(t12916: f64, t20837: f64, t5331: f64, t12910: f64, t21003: f64, t5245: f64, t5284: f64, t1214: f64, t20836: f64, t1250: f64, t12787: f64, t12809: f64, t17605: f64, t17623: f64, t17669: f64, t17674: f64, t17677: f64, t17682: f64, t17703: f64, t17747: f64, t20956: f64, t21040: f64, t3718: f64, t3720: f64, t5332: f64, t5340: f64, t57435: f64, t57449: f64, t57451: f64, t6421: f64) -> (f64, f64, f64) {
    let t70685 = t5331 * t12916 * t20837;
    let t70689 = t12910 * t12916 * t21003;
    let t70693 = t5245 * t5284;
    let t70712 = t20836 * t1214;
    let t70717 = 0.30488190661738479624e-2_f64 * t17605 * t17669 + 0.42874018118069736972e-3_f64 * t12910 * t3720 * t21040 * t17623 - 0.57165357490759649296e-3_f64 * t70685 + 0.28582678745379824648e-3_f64 * t57435 + 0.11433071498151929859e-2_f64 * t70689 - 0.19055119163586549765e-3_f64 * t57449 - 0.3811023832717309953e-3_f64 * t57451 - 0.85748036236139473944e-3_f64 * t3718 * t3720 * t70693 * t1250 - 0.12862205435420921092e-2_f64 * t17747 * t3720 * t20956 * t17703 + 0.47637797908966374414e-3_f64 * t5340 * t12787 * t6421 * t17677 - 0.23818898954483187207e-3_f64 * t5331 * t12787 * t6421 * t17682 + 0.15244095330869239812e-2_f64 * t17605 * t17674 + 0.85748036236139473944e-3_f64 * t12809 * t3720 * t5332 * t70712;
    (t70693, t70712, t70717)
}
