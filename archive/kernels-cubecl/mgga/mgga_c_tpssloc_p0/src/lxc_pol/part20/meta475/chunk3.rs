//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1952/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1952<F: Float>(t28: F, t265: F, t504: F, t13493: F, t14959: F, t15842: F, t1081: F, t1260: F, t12606: F, t13503: F, t13504: F, t13506: F, t1409: F, t1534: F, t1649: F, t1768: F, t2250: F, t2756: F, t3231: F, t3644: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t607: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t15844 = piecewise3::<F>(t505, t14959 + t15842, t13493);
    let t15856 = piecewise3::<F>(t401, t13493 * t28 / F::cast_from(2.0_f64) + t4324 * t1081 + t1534 * t3231 / F::cast_from(2.0_f64) + t2756 * t1649 / F::cast_from(2.0_f64) - t13503 - t13504 + t13506, t15844 * t52 / F::cast_from(2.0_f64) - t5099 * t607 - t1768 * t2250 / F::cast_from(2.0_f64) - t3644 * t1409 / F::cast_from(2.0_f64) - t1260 * t3966 - t506 * t12606 / F::cast_from(2.0_f64));
    (t15844, t15856)
}
