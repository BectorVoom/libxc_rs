//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 788/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk788(t5: f64, t25: f64, t265: f64, t394: f64, t24541: f64, t112: f64, t671: f64, t7408: f64, t2165: f64, t2363: f64, t23772: f64, t2116: f64, t2250: f64, t23309: f64, t40: f64, t607: f64, t7274: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t24542 = piecewise3(t8, 0.0_f64, t24541);
    let t24543 = t24542 * t112;
    let t24545 = t7408 * t671;
    let t24552 = t2165 * t2363;
    let t24555 = piecewise3(t395, 0.0_f64, t23772);
    let t24562 = piecewise3(t115, t23309, t24555 * t40 / 2.0_f64 + t7274 * t607 + t2116 * t2250 / 2.0_f64);
    (t24542, t24543, t24545, t24552, t24562)
}
