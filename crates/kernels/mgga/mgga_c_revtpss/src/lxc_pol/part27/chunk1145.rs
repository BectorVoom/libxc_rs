//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1145/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1145<F: Float>(t30: F, t265: F, t393: F, t94213: F, t10326: F, t2129: F, t2258: F, t26809: F, t45: F, t606: F, t7594: F, t93409: F, t12627: F, t2142: F, t12640: F, t26982: F, t3565: F, t7635: F, t1032: F, t3727: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t96848 = piecewise3(t394, 0.0, t94213);
    let t96858 = piecewise3(t120, t93409, t96848 * t45 / 2.0 + 3.0 / 2.0 * t26809 * t606 + 3.0 / 2.0 * t7594 * t2258 + t2129 * t10326 / 2.0);
    let t96861 = t12627 * t2142;
    let t96866 = t12640 * t2142;
    let t96870 = t26982 * t3565 * t7635;
    let t96873 = t3727 * t1032;
    (t96858, t96861, t96866, t96870, t96873)
}
