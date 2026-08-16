//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1063/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1063<F: Float>(t75748: F, t75756: F, t71628: F, t2329: F, t72109: F, t2344: F, t71229: F, t14581: F, t8526: F, t75758: F, t1364: F, t14567: F, t1632: F, t1635: F, t1668: F, t1685: F, t3204: F, t3207: F, t71615: F, t71619: F, t71620: F, t72: F, t75762: F, t75767: F, t77847: F, t77861: F, t77882: F, t77912: F, t77931: F, t77958: F, t77989: F, t78013: F, t78035: F, t78056: F, t78080: F, t78107: F, t78118: F, t78210: F, t78226: F, t78254: F, t82: F, t903: F) -> F {
    let t78271 = F::cast_from(0.79808624799933448875e-4_f64) * t75748;
    let t78272 = F::cast_from(0.212822999466489197e-4_f64) * t75756;
    let t78273 = F::cast_from(0.39914139006212695213e-1_f64) * t71628;
    let t78274 = t72109 * t2329;
    let t78275 = F::cast_from(0.13637330827122670864e-1_f64) * t78274;
    let t78276 = t71229 * t2344;
    let t78277 = F::cast_from(0.10227998120342003148e-1_f64) * t78276;
    let t78278 = t14581 * t8526;
    let t78279 = F::cast_from(0.10227998120342003148e-1_f64) * t78278;
    let t78280 = F::cast_from(0.14967802127329760705e-1_f64) * t75758;
    let t78282 = t72 * t82 * (t77847 + t77861 + t77882 + t77912 + t77931 + t77958 + t77989 + t78013 + t78035 + t78056 + t78080 + t78107 + t78118 + t78210 + t78226 + t78254) + t72 * t1685 * t3207 + F::cast_from(0.17961362552795712846e0_f64) * t903 * t3204 * t1632 - F::cast_from(0.23948483403727617128e0_f64) * t1364 * t3204 * t1635 - F::cast_from(0.2363e1_f64) * t1668 * t14567 + t71615 + t71619 - t71620 - t78271 - t78272 - t78273 + t78275 - t78277 - t78279 - t78280 - F::cast_from(0.58171619854173713846e-5_f64) * t75762 - t75767;
    t78282
}
