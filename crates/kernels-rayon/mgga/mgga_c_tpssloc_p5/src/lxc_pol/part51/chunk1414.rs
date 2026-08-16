//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1414/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1414(t25: f64, t265: f64, t394: f64, t121907: f64, t121949: f64, t121283: f64, t121798: f64, t121833: f64, t121865: f64, t1409: f64, t31478: f64, t33513: f64, t3966: f64, t40: f64, t607: f64, t8580: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t121950 = t121907 + t121949;
    let t121951 = piecewise3(t395, 0.0_f64, t121950);
    let t121958 = piecewise3(t115, t121283 + t121798 + t121833 + t121865, t121951 * t40 / 2.0_f64 + t31478 * t1409 / 2.0_f64 + t33513 * t607 / 2.0_f64 + t8580 * t3966 / 2.0_f64);
    (t121950, t121958)
}
