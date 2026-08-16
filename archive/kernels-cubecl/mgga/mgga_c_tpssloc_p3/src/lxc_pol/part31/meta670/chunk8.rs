//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1997/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1997<F: Float>(t28: F, t265: F, t504: F, t101938: F, t101981: F, t102012: F, t102048: F, t102087: F, t1409: F, t16558: F, t2071: F, t26862: F, t29189: F, t3966: F, t52: F, t5398: F, t607: F, t7150: F, t7884: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t102090 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t101938);
    let t102102 = piecewise3::<F>(t401, t101981 + t102012 + t102048 + t102087, t102090 * t52 / F::cast_from(2.0_f64) - t29189 * t607 / F::cast_from(2.0_f64) - t26862 * t1409 - t7884 * t3966 - t7150 * t5398 / F::cast_from(2.0_f64) - t2071 * t16558 / F::cast_from(2.0_f64));
    t102102
}
