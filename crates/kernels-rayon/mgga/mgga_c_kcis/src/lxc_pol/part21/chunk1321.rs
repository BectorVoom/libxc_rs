//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1321/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1321(t13314: f64, t303: f64, t356: f64, t26717: f64, t27895: f64, t46978: f64, t8033: f64, t2173: f64, t26753: f64, t2842: f64, t4556: f64, t1009: f64, t26697: f64, t26732: f64, t26767: f64, t27832: f64, t291: f64, t330: f64, t5314: f64, t93638: f64, t93653: f64, t93662: f64, t93664: f64) -> (f64, f64, f64, f64) {
    let t96298 = t303 * t356 * t13314;
    let t96302 = 0.61836467013888888889e-4_f64 * t27895 * t26717;
    let t96305 = t46978 * t8033;
    let t96306 = t2173 * t96305;
    let t96311 = t2842 * t26753 * t4556;
    let t96313 = -0.23168402777777777778e-3_f64 * t27832 * t26767 - 0.30891203703703703704e-3_f64 * t27832 * t26697 - 0.46336805555555555556e-3_f64 * t2173 * t5314 * t291 * t1009 * t330 - 0.61890573922526041668e-5_f64 * t93638 + 0.1621345679012345679e-1_f64 * t96298 + 0.46336805555555555556e-3_f64 * t93653 + t96302 - 0.41224311342592592593e-4_f64 * t93662 - 0.92754700520833333335e-4_f64 * t93664 - 0.15445601851851851852e-3_f64 * t96306 + 0.92754700520833333333e-4_f64 * t27895 * t26732 + 0.55273148148148148146e-2_f64 * t96311;
    (t96298, t96305, t96311, t96313)
}
