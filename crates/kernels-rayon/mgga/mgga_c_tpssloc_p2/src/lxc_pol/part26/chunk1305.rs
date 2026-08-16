//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1305/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1305(t22986: f64, t23270: f64, t2742: f64, t776: f64, t857: f64, t23273: f64, t81591: f64, t10112: f64, t10115: f64, t1912: f64, t23281: f64, t25168: f64, t25169: f64, t2720: f64, t41554: f64, t6627: f64, t6663: f64, t82087: f64, t82092: f64, t82099: f64, t82108: f64, t9590: f64) -> f64 {
    let t82113 = t22986 * t23270 * t857 * t2742 * t776;
    let t82115 = t81591 * t23273;
    let t82117 = -0.24674011002723396547e-1_f64 * t82087 - 0.9869604401089358619e-1_f64 * t82092 - 18.0_f64 * t25168 * t25169 * t10115 - 3.0_f64 * t41554 * t1912 + 0.78134368175290755733e-1_f64 * t82099 - 3.0_f64 * t9590 * t6663 - 6.0_f64 * t6627 * t10112 + 6.0_f64 * t23281 * t2720 - 0.74022033008170189643e-1_f64 * t82108 + 0.49348022005446793095e-1_f64 * t82113 - 0.23029076935875170111e0_f64 * t82115;
    t82117
}
