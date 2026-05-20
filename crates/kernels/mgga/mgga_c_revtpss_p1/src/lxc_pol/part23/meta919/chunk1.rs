//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2965/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2965<F: Float>(t23858: F, t3127: F, t3172: F, t23634: F, t1042: F, t11656: F, t11994: F, t15716: F, t15728: F, t15850: F, t1592: F, t15935: F, t1663: F, t19414: F, t19668: F, t19672: F, t19968: F, t23635: F, t23830: F, t23834: F, t23863: F, t23892: F, t247: F, t3116: F, t42669: F, t42973: F, t4803: F, t4834: F, t54492: F, t54982: F, t6312: F, t6327: F, t65712: F, t78550: F, t78554: F, t78561: F, t78564: F, t78570: F) -> F {
    let t78576 = t3127 * t3172 * t23858;
    let t78583 = t3127 * t3172 * t23634;
    let t78601 = F::cast_from(0.34299214494455789578e-2_f64) * t54492 * t6312 - F::cast_from(0.95275595817932748827e-3_f64) * t78550 + F::cast_from(0.7145669686344956162e-3_f64) * t15850 * t6327 + F::cast_from(0.51448821741683684368e-2_f64) * t54982 * t247 * t3116 * t78554 + F::cast_from(0.95275595817932748827e-4_f64) * t78561 - F::cast_from(0.47637797908966374413e-3_f64) * t78564 + F::cast_from(0.12862205435420921092e-2_f64) * t42669 * t23830 - F::cast_from(0.12862205435420921092e-2_f64) * t42973 * t23834 - F::cast_from(0.85748036236139473944e-3_f64) * t3127 * t1042 * t15935 * t78570 - F::cast_from(0.28582678745379824648e-3_f64) * t78576 - F::cast_from(0.45732285992607719437e-2_f64) * t15728 * t23863 - F::cast_from(0.45732285992607719437e-2_f64) * t11656 * t23635 + F::cast_from(0.57165357490759649296e-3_f64) * t78583 - F::cast_from(0.85748036236139473944e-3_f64) * t19968 * t4803 + F::cast_from(0.14291339372689912324e-2_f64) * t4834 * t19668 + F::cast_from(0.19055119163586549765e-2_f64) * t4834 * t19672 - F::cast_from(0.38586616306262763276e-2_f64) * t15716 * t1042 * t1663 * t19414 - F::cast_from(0.42874018118069736972e-3_f64) * t11994 * t23892 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t65712 * t1592;
    t78601
}
