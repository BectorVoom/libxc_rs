//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1041/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1041<F: Float>(t25: F, t1409: F, t1965: F, t25398: F, t25883: F, t3966: F, t40: F, t607: F, t6835: F, t7643: F, t28: F, t870: F, t4255: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t25890 = piecewise3::<F>(t115, t25398, t6835 * t1409 / F::cast_from(2.0_f64) + t1965 * t3966 / F::cast_from(2.0_f64) + t25883 * t40 / F::cast_from(2.0_f64) + t7643 * t607 / F::cast_from(2.0_f64));
    let t25891 = t870 * t28;
    let t25892 = t25891 * t4255;
    (t25890, t25892)
}
