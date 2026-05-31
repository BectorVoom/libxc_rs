//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1017/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1017<F: Float>(t33: F, t1469: F, t33896: F, t35008: F, t57: F, t8960: F, t34393: F, t118: F, t1502: F, t1843: F, t2127: F, t2163: F, t33664: F, t33666: F, t33669: F, t33916: F, t33920: F, t33977: F, t34429: F, t34434: F, t34444: F, t34447: F, t34449: F, t34464: F, t34874: F, t508: F, t8152: F, t8233: F, t8463: F, t8917: F, t8964: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t35013 = piecewise3::<F>(t400, t33896, -t8960 * t1469 / F::cast_from(2.0_f64) + t35008 * t57 / F::cast_from(2.0_f64));
    let t35014 = t34393 + t35013;
    let t35017 = -t118 * t35014 - t1502 * t8964 - t1843 * t8917 - F::cast_from(2.0_f64) * t2127 * t8233 - F::cast_from(2.0_f64) * t2163 * t8152 - t34874 * t508 - t33664 - t33666 + t33669 - t33916 + t33920 + t33977 - F::cast_from(4.0_f64) * t34429 - F::cast_from(4.0_f64) * t34434 - F::cast_from(4.0_f64) * t34444 - F::cast_from(4.0_f64) * t34447 - F::cast_from(4.0_f64) * t34449 + F::cast_from(6.0_f64) * t34464 - t8463;
    (t35014, t35017)
}
