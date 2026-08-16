//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1091/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1091(t1562: f64, t9639: f64, t10267: f64, t1668: f64, t1707: f64, t2211: f64, t2228: f64, t30344: f64, t43190: f64, t43204: f64, t43207: f64, t43211: f64, t43241: f64, t43817: f64, t45890: f64, t45896: f64, t45901: f64, t45905: f64, t45909: f64, t45911: f64, t45914: f64, t4601: f64, t530: f64, t739: f64, t903: f64, t9343: f64) -> f64 {
    let t48700 = t1562 * t9639;
    let t48706 = -t43190 + 0.35922725105591425692e0_f64 * t4601 * t10267 + 0.35922725105591425692e0_f64 * t903 * t2228 * t1707 - 0.5107751987195740728e-4_f64 * t45890 - 0.638468998399467591e-4_f64 * t45896 - 0.10215503974391481456e-3_f64 * t45901 + 0.30646511923174444368e-3_f64 * t45905 - 0.5107751987195740728e-3_f64 * t45909 + t43204 - t43207 + t43211 + 0.212822999466489197e-4_f64 * t45911 - 0.4726e1_f64 * t1668 * t9343 - 0.4726e1_f64 * t530 * t43817 - 0.4726e1_f64 * t48700 - t43241 + 0.5987120850931904282e-1_f64 * t45914 + 0.23948483403727617128e0_f64 * t739 * t2211 * t30344;
    t48706
}
