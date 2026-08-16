//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 226/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk226(t159: f64, t751: f64, t104: f64, t260: f64, t14: f64, t1: f64, t269: f64, t546: f64, t106: f64, t257: f64, t748: f64, t10: f64, t103: f64, t164: f64, t266: f64, t303: f64, t304: f64, t758: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t849 = t751 * t159;
    let t852 = t260 * t104;
    let t853 = t852 * t14;
    let t854 = t269 * t1;
    let t855 = t854 * t546;
    let t858 = t106 * t257;
    let t859 = t858 * t748;
    let t868 = 0.58998125e-2_f64 * t849 * t304 - 0.11799625e-1_f64 * t853 * t855 - 0.58998125e-2_f64 * t303 * t859 - 0.14341111111111111111e-1_f64 * t103 * t10 * t266 - 0.21511666666666666667e-1_f64 * t103 * t164 * t758;
    (t849, t852, t853, t854, t855, t858, t859, t868)
}
