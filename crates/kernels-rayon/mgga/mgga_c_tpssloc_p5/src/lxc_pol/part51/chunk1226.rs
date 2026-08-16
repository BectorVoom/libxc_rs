//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1226/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1226(t33379: f64, t6646: f64, t1888: f64, t1894: f64, t7823: f64, t214: f64, t1880: f64, t1510: f64, t31394: f64, t31353: f64, t31355: f64, t31359: f64, t32835: f64, t32838: f64, t32841: f64, t32845: f64, t32847: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33380 = t6646 * t33379;
    let t33381 = t1888 * t33380;
    let t33383 = t1894 * t7823;
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    let t33388 = t31394 * t1510;
    let t33395 = -t31353 - 0.96894614625936938046e-2_f64 * t32835 - t31355 - 0.16149102437656156341e-2_f64 * t32838 + t32841 / 768.0_f64 - t32845 / 768.0_f64 - t31359 - t32847 / 192.0_f64;
    (t33380, t33381, t33383, t33384, t33385, t33388, t33395)
}
