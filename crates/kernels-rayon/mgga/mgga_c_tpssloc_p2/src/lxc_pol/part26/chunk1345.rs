//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1345/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1345(t25: f64, t265: f64, t394: f64, t83543: f64, t2116: f64, t2250: f64, t24555: f64, t40: f64, t607: f64, t7274: f64, t82334: f64, t9258: f64, t1240: f64, t3630: f64, t11588: f64, t2127: f64, t221: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t85617 = piecewise3(t395, 0.0_f64, t83543);
    let t85627 = piecewise3(t115, t82334, t85617 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t24555 * t607 + 3.0_f64 / 2.0_f64 * t7274 * t2250 + t2116 * t9258 / 2.0_f64);
    let t85628 = t1240 * t3630;
    let t85639 = t2127 * t221 * t11588;
    (t85627, t85628, t85639)
}
