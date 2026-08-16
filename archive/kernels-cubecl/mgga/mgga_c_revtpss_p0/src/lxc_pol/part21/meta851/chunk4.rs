//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3201/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3201<F: Float>(t45608: F, t58919: F, t1042: F, t1248: F, t1261: F, t12780: F, t12788: F, t12800: F, t12805: F, t12866: F, t12938: F, t12956: F, t13043: F, t13045: F, t16746: F, t17199: F, t17232: F, t17401: F, t17541: F, t17569: F, t17654: F, t17661: F, t17729: F, t17799: F, t20945: F, t2251: F, t3362: F, t3604: F, t3625: F, t3626: F, t3647: F, t3720: F, t44230: F, t44260: F, t44797: F, t5051: F, t5270: F, t5299: F, t53474: F, t5402: F, t5405: F, t56903: F, t58921: F, t58960: F, t58969: F, t58975: F, t58983: F, t58997: F) -> F {
    let t59001 = t45608 * t58919;
    let t59007 = F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17661 * t12780 - F::cast_from(0.7145669686344956162e-3_f64) * t17569 * t12938 + F::cast_from(0.14291339372689912324e-2_f64) * t17654 * t20945 * t3604 * t1248 * t3362 * t2251 - F::cast_from(0.7145669686344956162e-3_f64) * t12866 * t58960 * t12788 - F::cast_from(0.17149607247227894789e-2_f64) * t17654 * t17799 * t56903 - t44797 + F::cast_from(0.42874018118069736972e-3_f64) * t12956 * t17541 + F::cast_from(0.85748036236139473944e-3_f64) * t17729 * t3626 * t5051 * t58969 - F::cast_from(0.17149607247227894789e-2_f64) * t58975 - F::cast_from(0.85748036236139473944e-3_f64) * t12800 * t5270 - F::cast_from(0.17149607247227894789e-2_f64) * t3647 * t17232 - F::cast_from(0.85748036236139473944e-3_f64) * t3647 * t17199 + F::cast_from(0.23289590088828005269e-2_f64) * t1261 * t1042 * t58983 * t53474 + F::cast_from(0.42874018118069736972e-3_f64) * t44260 * t5299 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t16746 * t5405 - F::cast_from(0.42874018118069736972e-3_f64) * t44230 * t5402 + F::cast_from(0.17149607247227894789e-2_f64) * t58997 - F::cast_from(0.64311027177104605458e-3_f64) * t17401 * t12805 - F::cast_from(0.77173232612525526552e-2_f64) * t59001 * t3720 * t58921 * t13043 * t13045;
    t59007
}
