//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1382/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1382<F: Float>(t17483: F, t3720: F, t1222: F, t12809: F, t12882: F, t12887: F, t12893: F, t12895: F, t12900: F, t12902: F, t12905: F, t17474: F, t17476: F, t17479: F) -> F {
    let t17484 = t3720 * t17483;
    let t17493 = t17474 - F::new(7.0) / F::new(648.0) * t1222 * t17476 + t1222 * t17479 / F::new(36.0) + F::cast_from(0.21437009059034868486e-3_f64) * t12809 * t17484 + F::cast_from(0.63517063878621832551e-4_f64) * t12882 + F::cast_from(0.15879265969655458138e-3_f64) * t12887 - F::cast_from(0.95275595817932748826e-4_f64) * t12893 + F::cast_from(0.14291339372689912324e-3_f64) * t12895 + t12900 + F::cast_from(0.28582678745379824648e-3_f64) * t12902 - F::cast_from(0.95275595817932748826e-4_f64) * t12905;
    t17493
}
