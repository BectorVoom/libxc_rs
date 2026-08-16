//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 669/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk669<F: Float>(t28: F, t265: F, t504: F, t4324: F, t5098: F, t1081: F, t1260: F, t1409: F, t1534: F, t1649: F, t1768: F, t3966: F, t4332: F, t506: F, t52: F, t607: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t5099 = piecewise3::<F>(t505, t5098, t4324);
    let t5106 = piecewise3::<F>(t401, t4324 * t28 / F::cast_from(2.0_f64) + t1534 * t1081 / F::cast_from(2.0_f64) + t873 * t1649 / F::cast_from(2.0_f64) - t4332, -t1260 * t1409 / F::cast_from(2.0_f64) - t1768 * t607 / F::cast_from(2.0_f64) - t506 * t3966 / F::cast_from(2.0_f64) + t5099 * t52 / F::cast_from(2.0_f64));
    t5106
}
