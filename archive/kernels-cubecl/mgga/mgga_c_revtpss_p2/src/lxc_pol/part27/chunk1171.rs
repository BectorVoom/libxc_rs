//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1171/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1171<F: Float>(t33: F, t2159: F, t2258: F, t25791: F, t27048: F, t57: F, t606: F, t7677: F, t26816: F, t116: F, t7583: F, t2371: F, t25812: F, t25814: F, t25816: F, t25818: F, t25820: F, t25834: F, t26800: F, t26804: F, t670: F, t7586: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t27055 = piecewise3::<F>(t400, t25791, t27048 * t57 / F::cast_from(2.0_f64) - t7677 * t606 - t2159 * t2258 / F::cast_from(2.0_f64));
    let t27056 = t26816 + t27055;
    let t27060 = t7583 * t116;
    let t27066 = F::cast_from(2.0_f64) * t2371 * t7586 + F::cast_from(4.0_f64) * t27060 * t670 + t25812 + t25814 + t25816 + t25818 + t25820 + t25834 + t26800 + F::cast_from(2.0_f64) * t26804;
    (t27056, t27060, t27066)
}
