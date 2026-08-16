//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1284/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1284(t125295: f64, t2122: f64, t225: f64, t34278: f64, t117930: f64, t118050: f64, t118052: f64, t1186: f64, t1190: f64, t1238: f64, t1251: f64, t1252: f64, t14980: f64, t1716: f64, t2144: f64, t2154: f64, t24880: f64, t27395: f64, t27453: f64, t27741: f64, t27784: f64, t27785: f64, t32480: f64, t32482: f64, t32504: f64, t32524: f64, t34277: f64, t34305: f64, t34318: f64, t3487: f64, t3598: f64, t460: f64, t4945: f64, t498: f64, t5060: f64, t7283: f64, t7286: f64, t7999: f64, t8061: f64, t8898: f64, t94395: f64) -> f64 {
    let t125596 = t2122 * t125295;
    let t125613 = t34278 * t225;
    let t125624 = 2.0_f64 * t3487 * t34318 + t1190 * t34277 * t498 - t14980 * t8898 + 2.0_f64 * t32482 * t5060 - 0.14621636149762012769e-1_f64 * t94395 * t32524 + 0.54831135561607547883e-2_f64 * t118050 - t4945 * t32480 + 4.0_f64 * t24880 * t8061 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t125596 + 4.0_f64 * t1238 * t3598 * t2154 * t27741 - 0.43864908449286038307e-1_f64 * t7999 * t32504 + 2.0_f64 * t1238 * t3598 * t34305 * t1251 - 12.0_f64 * t27784 * t27785 * t27395 - t125613 * t1252 + 0.16449340668482264365e-1_f64 * t7283 * t1716 * t117930 + 0.10966227112321509577e-1_f64 * t118052 - 0.16449340668482264365e-1_f64 * t7283 * t27453 * t460 * t2144 * t7286;
    t125624
}
