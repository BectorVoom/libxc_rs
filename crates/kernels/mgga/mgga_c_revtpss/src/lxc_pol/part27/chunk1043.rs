//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1043/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1043<F: Float>(t33: F, t265: F, t502: F, t26968: F, t27032: F, t3801: F, t7669: F, t12587: F, t2155: F, t1298: F, t1300: F, t198: F, t25743: F, t336: F, t3794: F, t3798: F, t5023: F, t7673: F, t2159: F, t2258: F, t25791: F, t57: F, t606: F, t7677: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t27033 = t26968 + t27032;
    let t27037 = t7669 * t3801;
    let t27041 = t2155 * t12587;
    let t27048 = piecewise3(t503, t1300 * t198 * t27033 * t336 - 2.0 * t1298 * t27037 * t5023 + 2.0 * t27041 * t3798 * t5023 - t3794 * t5023 * t7673, t25743);
    let t27055 = piecewise3(t400, t25791, t27048 * t57 / 2.0 - t7677 * t606 - t2159 * t2258 / 2.0);
    (t27033, t27037, t27041, t27048, t27055)
}
