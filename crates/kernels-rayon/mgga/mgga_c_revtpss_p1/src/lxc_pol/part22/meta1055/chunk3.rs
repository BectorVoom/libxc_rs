//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3733/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3733(t11249: f64, t6622: f64, t1214: f64, t1250: f64, t12866: f64, t1715: f64, t17353: f64, t17515: f64, t17623: f64, t17709: f64, t17711: f64, t17747: f64, t17748: f64, t20795: f64, t20933: f64, t20934: f64, t3584: f64, t3720: f64, t44561: f64, t44607: f64, t44952: f64, t5056: f64, t56981: f64, t57604: f64, t57615: f64, t57635: f64, t57660: f64, t57687: f64) -> (f64, f64) {
    let t70890 = t6622 * t11249;
    let t70907 = 0.28582678745379824648e-3_f64 * t57604 + 0.1270341277572436651e-3_f64 * t57615 + 0.57165357490759649296e-3_f64 * t44561 * t20934 + 0.57165357490759649296e-3_f64 * t12866 * t56981 * t20933 + 0.57165357490759649296e-3_f64 * t12866 * t17353 * t1250 * t5056 * t1214 + 0.28582678745379824648e-3_f64 * t12866 * t17353 * t1250 * t1715 * t3584 + 0.12862205435420921092e-2_f64 * t17709 * t3720 * t70890 * t17711 - 0.12862205435420921092e-2_f64 * t17747 * t3720 * t70890 * t17748 - t44607 - 0.3811023832717309953e-3_f64 * t57635 - 0.42874018118069736972e-3_f64 * t44952 * t3720 * t20795 * t17623 - 0.30488190661738479624e-2_f64 * t57660 * t17515 - 5.0_f64 / 1944.0_f64 * t57687;
    (t70890, t70907)
}
