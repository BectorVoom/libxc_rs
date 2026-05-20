//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3102/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3102<F: Float>(t1214: F, t3611: F, t12831: F, t17395: F, t12702: F, t17350: F, t1263: F, t372: F, t5284: F, t1250: F, t12809: F, t12862: F, t12866: F, t13069: F, t16696: F, t16756: F, t17353: F, t17396: F, t17482: F, t17512: F, t17649: F, t17654: F, t17657: F, t17658: F, t17677: F, t17682: F, t2251: F, t2258: F, t3367: F, t3720: F, t3723: F, t44273: F, t44276: F, t44278: F, t44280: F, t44283: F, t44286: F, t44289: F, t44510: F, t44517: F, t44952: F, t5287: F, t5297: F) -> (F, F) {
    let t56947 = t3611 * t1214;
    let t56953 = t12831 * t17395;
    let t56977 = t12702 * t17350;
    let t56981 = t372 * t1263 * t5284;
    let t56985 = -F::cast_from(0.14291339372689912324e-3_f64) * t44273 + F::cast_from(0.14291339372689912324e-3_f64) * t44276 + F::cast_from(0.42874018118069736972e-3_f64) * t44278 + F::cast_from(0.64311027177104605458e-3_f64) * t13069 * t5287 + F::cast_from(0.42874018118069736972e-3_f64) * t44280 - F::cast_from(0.42874018118069736972e-3_f64) * t44283 - F::cast_from(0.47637797908966374413e-3_f64) * t44286 + F::cast_from(0.12862205435420921092e-2_f64) * t12809 * t3720 * t16756 * t16696 - F::cast_from(0.12862205435420921092e-2_f64) * t44952 * t3720 * t17482 * t56947 - F::cast_from(0.57165357490759649295e-3_f64) * t44289 + F::cast_from(0.68598428988911579154e-2_f64) * t56953 * t3723 + F::cast_from(0.34299214494455789577e-2_f64) * t17396 * t12862 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17353 * t1250 * t17512 * t2258 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17353 * t1250 * t1214 * t3367 * t2251 + F::cast_from(0.85748036236139473944e-3_f64) * t44510 * t17649 * t5297 * t17677 - F::cast_from(0.42874018118069736972e-3_f64) * t44517 * t17649 * t5297 * t17682 - F::cast_from(0.17149607247227894789e-2_f64) * t56977 * t17658 - F::cast_from(0.17149607247227894789e-2_f64) * t17654 * t56981 * t17657;
    (t56981, t56985)
}
