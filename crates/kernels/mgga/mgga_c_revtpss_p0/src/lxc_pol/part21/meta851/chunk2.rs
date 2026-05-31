//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3199/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3199<F: Float>(t3666: F, t5390: F, t1042: F, t1261: F, t12841: F, t12842: F, t12846: F, t12847: F, t12866: F, t13043: F, t1469: F, t17513: F, t17633: F, t17644: F, t17661: F, t17693: F, t17700: F, t21014: F, t21017: F, t3625: F, t3626: F, t3647: F, t3714: F, t372: F, t3720: F, t44510: F, t44517: F, t44535: F, t44786: F, t44789: F, t44792: F, t5268: F, t5302: F, t5333: F, t53459: F, t53464: F, t57622: F, t58872: F, t58889: F, t58897: F, t58899: F, t58909: F, t58920: F, t58921: F) -> F {
    let t58927 = t3666 * t5390;
    let t58948 = F::cast_from(0.95275595817932748825e-4_f64) * t58889 + F::cast_from(0.45732285992607719436e-2_f64) * t21014 * t12842 - F::cast_from(0.22866142996303859718e-2_f64) * t21017 * t12847 + F::cast_from(0.57165357490759649295e-3_f64) * t58897 + F::cast_from(0.42874018118069736973e-2_f64) * t17693 * t58899 * t57622 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t3626 * t17633 * t17644 + t44786 / F::cast_from(216.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t44517 * t58909 * t5333 * t17513 + F::cast_from(0.17149607247227894789e-2_f64) * t12866 * t372 * t5268 * t1469 * t58872 + F::cast_from(0.51448821741683684368e-2_f64) * t58920 * t3720 * t58921 * t44535 * t13043 - F::cast_from(0.45732285992607719436e-2_f64) * t58927 * t3714 + F::cast_from(0.14291339372689912324e-2_f64) * t3647 * t17700 + F::cast_from(0.47637797908966374414e-3_f64) * t44789 + F::cast_from(0.28582678745379824648e-3_f64) * t44792 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t1042 * t5302 * t53459 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t1042 * t5302 * t53464 + F::cast_from(0.85748036236139473944e-3_f64) * t44510 * t17661 * t12841 - F::cast_from(0.42874018118069736972e-3_f64) * t44517 * t17661 * t12846;
    t58948
}
