//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1110/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1110<F: Float>(t28: F, t265: F, t504: F, t26806: F, t1409: F, t2071: F, t26861: F, t3966: F, t52: F, t607: F, t7150: F, t7884: F, t26814: F, t19577: F, t24432: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t26862 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t26806);
    let t26869 = piecewise3::<F>(t401, t26861, -t7150 * t1409 / F::cast_from(2.0_f64) - t2071 * t3966 / F::cast_from(2.0_f64) + t26862 * t52 / F::cast_from(2.0_f64) - t7884 * t607 / F::cast_from(2.0_f64));
    let t26870 = t26814 + t26869;
    let t26872 = t24432 * t19577;
    (t26870, t26872)
}
