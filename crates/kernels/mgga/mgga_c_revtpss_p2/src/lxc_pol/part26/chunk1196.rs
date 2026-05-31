//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1196/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1196<F: Float>(t1113: F, t1940: F, t2071: F, t2403: F, t25752: F, t25760: F, t25763: F, t25778: F, t25781: F, t26581: F, t26585: F, t3351: F, t4541: F, t7200: F, t7207: F, t7428: F, t7432: F, t9357: F, t94255: F, t94262: F, t94286: F, t94312: F, t94320: F, t95511: F, t95527: F, t95964: F, t95976: F) -> F {
    let t96166 = F::cast_from(9.0_f64) * t4541 * t2071 * t94262 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2403 * t26581 * t7200 + F::cast_from(9.0_f64) * t2403 * t7428 * t25763 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t26581 * t1113 - F::cast_from(9.0_f64) * t95511 * t25760 + t1940 * t2071 * t9357 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t1940 * t95964 * t94312 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t94320 - F::cast_from(3.0_f64) * t1940 * t26585 * t25781 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7432 * t94286 + F::cast_from(3.0_f64) * t1940 * t95976 * t25778 + F::cast_from(9.0_f64) * t4541 * t7428 * t25752 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t7428 * t3351 - t1940 * t7432 * t94255 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1940 * t95527 * t7207;
    t96166
}
