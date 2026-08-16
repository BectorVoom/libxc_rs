//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1377/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1377<F: Float>(t25: F, t265: F, t394: F, t41606: F, t42274: F, t43627: F, t43641: F, t43642: F, t10150: F, t1074: F, t11105: F, t2249: F, t2250: F, t2756: F, t3220: F, t39109: F, t39110: F, t396: F, t40: F, t606: F, t607: F, t873: F, t9257: F, t9258: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t43645 = piecewise3::<F>(t395, t42274 + t43627 + t43641 + t43642, t41606);
    let t43657 = piecewise3::<F>(t115, t41606 * t25 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t10150 * t606 + F::cast_from(3.0_f64) * t2756 * t2249 + F::cast_from(2.0_f64) * t873 * t9257 + t265 * t39109 / F::cast_from(2.0_f64), t43645 * t40 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t11105 * t607 + F::cast_from(3.0_f64) * t3220 * t2250 + F::cast_from(2.0_f64) * t1074 * t9258 + t396 * t39110 / F::cast_from(2.0_f64));
    t43657
}
