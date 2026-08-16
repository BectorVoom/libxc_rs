//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1266/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1266(t121131: f64, t128790: f64, t121249: f64, t122443: f64, t122493: f64, t122494: f64, t122496: f64, t125868: f64, t27853: f64, t27858: f64, t32690: f64, t32726: f64, t34204: f64, t7308: f64, t7921: f64, t7930: f64) -> f64 {
    let t128812 = t121131 * t128790;
    let t128826 = 0.37645955677973955999e-4_f64 * t121249 + 0.42839803248826764462e-1_f64 * t128812 + 0.17347256376410398924e1_f64 * t122443 * t7921 - t122493 + t122494 - 0.8673628188205199462e0_f64 * t34204 * t7308 - 0.8673628188205199462e0_f64 * t32726 * t7930 + 0.25389723392137995738e-1_f64 * t122496 + 0.7437465841810202164e-3_f64 * t125868 + 0.8673628188205199462e0_f64 * t32690 * t27853 + 0.8673628188205199462e0_f64 * t32690 * t27858;
    t128826
}
