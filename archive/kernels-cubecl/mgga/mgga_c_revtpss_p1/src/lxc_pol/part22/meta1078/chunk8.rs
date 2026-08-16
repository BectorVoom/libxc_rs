//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3868/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3868<F: Float>(t6876: F, t9909: F, t22026: F, t46929: F, t808: F, t22135: F, t9744: F, t1353: F, t13716: F, t1410: F, t1868: F, t22040: F, t3889: F, t3944: F, t4012: F, t46723: F, t46741: F, t46757: F, t48637: F, t48645: F, t48655: F, t6836: F, t800: F, t828: F, t9942: F) -> F {
    let t74358 = t9909 * t6876;
    let t74362 = t46929 * t808 * t22026;
    let t74364 = t9744 * t22135;
    let t74375 = -F::cast_from(0.25724410870841842183e-1_f64) * t1410 * t9942 * t828 * t6836 * t3889 + F::cast_from(0.85748036236139473944e-2_f64) * t1410 * t4012 * t828 * t1868 * t13716 - F::cast_from(0.56688979511669985553e-2_f64) * t74358 + F::cast_from(0.75585306015559980738e-1_f64) * t46723 - F::cast_from(0.50820002809285328225e-5_f64) * t74362 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t74364 + t3944 * t800 * t22040 * t1353 / F::cast_from(8.0_f64) - F::cast_from(0.10841600599314203354e-2_f64) * t46741 + F::cast_from(0.54208002996571016772e-3_f64) * t48637 + F::cast_from(0.90702367218671976886e-1_f64) * t48645 + F::cast_from(0.10164000561857065645e-3_f64) * t48655 - F::cast_from(0.45178982497454656791e-5_f64) * t46757;
    t74375
}
