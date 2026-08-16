//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1263/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1263<F: Float>(t12918: F, t1338: F, t1348: F, t10533: F, t11302: F, t11314: F, t12348: F, t12355: F, t12683: F, t2438: F, t35220: F, t3549: F, t3556: F, t3675: F, t38958: F, t38971: F, t38976: F, t42101: F, t42121: F, t42757: F, t9760: F) -> F {
    let t44855 = t1338 * t12918;
    let t44858 = t1348 * t12918;
    let t44873 = -F::cast_from(0.354375e1_f64) * t38976 * t42757 - F::cast_from(0.42e1_f64) * t42121 * t3675 - F::cast_from(0.42e1_f64) * t12348 * t9760 - F::cast_from(0.945e1_f64) * t38958 * t12683 - F::cast_from(0.21e1_f64) * t11302 * t10533 - F::cast_from(0.21e1_f64) * t3549 * t35220 - F::cast_from(0.21e1_f64) * t44855 * t2438 - F::cast_from(0.1575e1_f64) * t44858 * t2438 - F::cast_from(0.315e1_f64) * t42101 * t3675 - F::cast_from(0.315e1_f64) * t12355 * t9760 - F::cast_from(0.1575e1_f64) * t11314 * t10533 - F::cast_from(0.1575e1_f64) * t3556 * t35220 - F::cast_from(0.23625e1_f64) * t38971 * t12683 - F::cast_from(0.63e1_f64) * t11314 * t12683;
    t44873
}
