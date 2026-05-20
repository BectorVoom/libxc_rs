//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3733/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3733<F: Float>(t11249: F, t6622: F, t1214: F, t1250: F, t12866: F, t1715: F, t17353: F, t17515: F, t17623: F, t17709: F, t17711: F, t17747: F, t17748: F, t20795: F, t20933: F, t20934: F, t3584: F, t3720: F, t44561: F, t44607: F, t44952: F, t5056: F, t56981: F, t57604: F, t57615: F, t57635: F, t57660: F, t57687: F) -> (F, F) {
    let t70890 = t6622 * t11249;
    let t70907 = F::cast_from(0.28582678745379824648e-3_f64) * t57604 + F::cast_from(0.1270341277572436651e-3_f64) * t57615 + F::cast_from(0.57165357490759649296e-3_f64) * t44561 * t20934 + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t56981 * t20933 + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t17353 * t1250 * t5056 * t1214 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17353 * t1250 * t1715 * t3584 + F::cast_from(0.12862205435420921092e-2_f64) * t17709 * t3720 * t70890 * t17711 - F::cast_from(0.12862205435420921092e-2_f64) * t17747 * t3720 * t70890 * t17748 - t44607 - F::cast_from(0.3811023832717309953e-3_f64) * t57635 - F::cast_from(0.42874018118069736972e-3_f64) * t44952 * t3720 * t20795 * t17623 - F::cast_from(0.30488190661738479624e-2_f64) * t57660 * t17515 - F::new(5.0) / F::new(1944.0) * t57687;
    (t70890, t70907)
}
