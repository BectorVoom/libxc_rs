//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1200/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1200(t126089: f64, t119790: f64, t121806: f64, t121810: f64, t121815: f64, t126081: f64, t126083: f64, t126085: f64, t126087: f64, t126095: f64, t1955: f64, t1959: f64, t28340: f64) -> f64 {
    let t127620 = 0.13223814266738539448e-3_f64 * t126089;
    let t127628 = -0.29749863367240808656e-2_f64 * t126081 + 0.7437465841810202164e-3_f64 * t126083 + 0.7437465841810202164e-3_f64 * t126085 - 0.74374658418102021639e-4_f64 * t126087 + t127620 - 0.25702851531048074406e-1_f64 * t121806 - 0.8673628188205199462e0_f64 * t1955 * t28340 * t1959 + 0.28559868832551176308e-1_f64 * t121810 + t119790 + 0.14456046980341999104e-1_f64 * t121815 + 0.56468933516960933999e-3_f64 * t126095;
    t127628
}
