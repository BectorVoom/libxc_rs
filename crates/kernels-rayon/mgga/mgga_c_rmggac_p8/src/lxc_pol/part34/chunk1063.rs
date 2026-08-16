//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1063/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1063(t75748: f64, t75756: f64, t71628: f64, t2329: f64, t72109: f64, t2344: f64, t71229: f64, t14581: f64, t8526: f64, t75758: f64, t1364: f64, t14567: f64, t1632: f64, t1635: f64, t1668: f64, t1685: f64, t3204: f64, t3207: f64, t71615: f64, t71619: f64, t71620: f64, t72: f64, t75762: f64, t75767: f64, t77847: f64, t77861: f64, t77882: f64, t77912: f64, t77931: f64, t77958: f64, t77989: f64, t78013: f64, t78035: f64, t78056: f64, t78080: f64, t78107: f64, t78118: f64, t78210: f64, t78226: f64, t78254: f64, t82: f64, t903: f64) -> f64 {
    let t78271 = 0.79808624799933448875e-4_f64 * t75748;
    let t78272 = 0.212822999466489197e-4_f64 * t75756;
    let t78273 = 0.39914139006212695213e-1_f64 * t71628;
    let t78274 = t72109 * t2329;
    let t78275 = 0.13637330827122670864e-1_f64 * t78274;
    let t78276 = t71229 * t2344;
    let t78277 = 0.10227998120342003148e-1_f64 * t78276;
    let t78278 = t14581 * t8526;
    let t78279 = 0.10227998120342003148e-1_f64 * t78278;
    let t78280 = 0.14967802127329760705e-1_f64 * t75758;
    let t78282 = t72 * t82 * (t77847 + t77861 + t77882 + t77912 + t77931 + t77958 + t77989 + t78013 + t78035 + t78056 + t78080 + t78107 + t78118 + t78210 + t78226 + t78254) + t72 * t1685 * t3207 + 0.17961362552795712846e0_f64 * t903 * t3204 * t1632 - 0.23948483403727617128e0_f64 * t1364 * t3204 * t1635 - 0.2363e1_f64 * t1668 * t14567 + t71615 + t71619 - t71620 - t78271 - t78272 - t78273 + t78275 - t78277 - t78279 - t78280 - 0.58171619854173713846e-5_f64 * t75762 - t75767;
    t78282
}
