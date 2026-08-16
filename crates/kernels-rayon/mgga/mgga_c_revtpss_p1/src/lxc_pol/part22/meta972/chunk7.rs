//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3259/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3259(t10886: f64, t18599: f64, t808: f64, t1544: f64, t1559: f64, t40834: f64, t854: f64, t10770: f64, t14676: f64, t18426: f64, t18637: f64, t2394: f64, t2723: f64, t2745: f64, t2747: f64, t40594: f64, t40600: f64, t40607: f64, t40611: f64, t4362: f64, t50634: f64, t50643: f64, t50673: f64, t50681: f64) -> (f64, f64) {
    let t61833 = t10886 * t808 * t18599;
    let t61837 = t1559 * t1544;
    let t61839 = t40834 * t854 * t61837;
    let t61852 = 0.90702367218671976884e-1_f64 * t50634 - 0.25410001404642664112e-4_f64 * t50643 - 0.50820002809285328225e-4_f64 * t61833 + 0.45351183609335988442e-1_f64 * t40594 + 0.10164000561857065645e-4_f64 * t40600 + t40607 - t40611 - 0.2032800112371413129e-4_f64 * t61839 + 0.50820002809285328224e-4_f64 * t50673 - 0.10841600599314203354e-2_f64 * t50681 + 0.85748036236139473945e-2_f64 * t4362 * t10770 * t18426 * t2723 * t2394 + 0.34299214494455789578e-2_f64 * t2745 * t2747 * t14676 * t18637;
    (t61837, t61852)
}
