//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3197/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3197<F: Float>(t1174: F, t18225: F, t3431: F, t18221: F, t15522: F, t4889: F, t11668: F, t11678: F, t1177: F, t15686: F, t3248: F, t3252: F, t3440: F, t3494: F, t3577: F, t3578: F, t52893: F, t53270: F, t53272: F, t53274: F, t53287: F, t53291: F, t5979: F, t6225: F, t63368: F, t63410: F, t64990: F) -> F {
    let t66449 = t1174 * t3431 * t18225;
    let t66452 = t1174 * t3431 * t18221;
    let t66458 = t4889 * t15522;
    let t66480 = t53270 / F::cast_from(324.0_f64) - t53272 / F::cast_from(3456.0_f64) - t53274 / F::cast_from(972.0_f64) - t53287 / F::cast_from(1728.0_f64) - t66449 / F::cast_from(108.0_f64) - t66452 / F::cast_from(72.0_f64) - t1174 * t1177 * t63410 / F::cast_from(72.0_f64) - t53291 / F::cast_from(1728.0_f64) - F::cast_from(4.0_f64) / F::cast_from(243.0_f64) * t66458 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t4889 * t15686 + t1174 * t3440 * t63368 / F::cast_from(36.0_f64) - t3577 * t3578 * t5979 * t3494 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(576.0_f64) * t52893 * t11668 * t64990 - t11678 * t3578 * t6225 * t3252 / F::cast_from(2304.0_f64) - t11678 * t3578 * t6225 * t3248 / F::cast_from(1152.0_f64);
    t66480
}
