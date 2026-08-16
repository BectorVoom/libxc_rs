//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3868/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3868(t6876: f64, t9909: f64, t22026: f64, t46929: f64, t808: f64, t22135: f64, t9744: f64, t1353: f64, t13716: f64, t1410: f64, t1868: f64, t22040: f64, t3889: f64, t3944: f64, t4012: f64, t46723: f64, t46741: f64, t46757: f64, t48637: f64, t48645: f64, t48655: f64, t6836: f64, t800: f64, t828: f64, t9942: f64) -> f64 {
    let t74358 = t9909 * t6876;
    let t74362 = t46929 * t808 * t22026;
    let t74364 = t9744 * t22135;
    let t74375 = -0.25724410870841842183e-1_f64 * t1410 * t9942 * t828 * t6836 * t3889 + 0.85748036236139473944e-2_f64 * t1410 * t4012 * t828 * t1868 * t13716 - 0.56688979511669985553e-2_f64 * t74358 + 0.75585306015559980738e-1_f64 * t46723 - 0.50820002809285328225e-5_f64 * t74362 - 7.0_f64 / 24.0_f64 * t74364 + t3944 * t800 * t22040 * t1353 / 8.0_f64 - 0.10841600599314203354e-2_f64 * t46741 + 0.54208002996571016772e-3_f64 * t48637 + 0.90702367218671976886e-1_f64 * t48645 + 0.10164000561857065645e-3_f64 * t48655 - 0.45178982497454656791e-5_f64 * t46757;
    t74375
}
