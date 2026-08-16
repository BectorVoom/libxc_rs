//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 644/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk644(t120: f64, t1824: f64, t1351: f64, t3792: f64, t5248: f64, t1827: f64, t3799: f64, t1315: f64, t1354: f64, t1369: f64, t3733: f64, t3762: f64, t3763: f64, t3778: f64, t5220: f64, t5223: f64, t5227: f64, t5231: f64, t5235: f64, t5238: f64, t5240: f64, t5246: f64, t559: f64) -> (f64, f64, f64, f64) {
    let t5249 = t120 * t1824;
    let t5250 = t3792 * t1351;
    let t5252 = t5248 * t5249 * t5250;
    let t5255 = t3799 * t1827;
    let t5257 = t3762 + 7.0_f64 / 144.0_f64 * t3763 + 7.0_f64 / 144.0_f64 * t5220 + t3733 * t5223 / 16.0_f64 - t1315 * t5227 / 48.0_f64 + t5231 * t559 / 3072.0_f64 - t5235 * t1354 / 3072.0_f64 - 7.0_f64 / 4608.0_f64 * t5238 - t5240 * t1369 / 768.0_f64 - t3778 * t1827 / 3072.0_f64 + t5246 * t5252 / 1536.0_f64 + 7.0_f64 / 4608.0_f64 * t5255;
    (t5249, t5250, t5252, t5257)
}
