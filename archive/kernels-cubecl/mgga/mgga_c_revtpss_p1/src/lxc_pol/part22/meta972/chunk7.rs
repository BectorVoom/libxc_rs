//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3259/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3259<F: Float>(t10886: F, t18599: F, t808: F, t1544: F, t1559: F, t40834: F, t854: F, t10770: F, t14676: F, t18426: F, t18637: F, t2394: F, t2723: F, t2745: F, t2747: F, t40594: F, t40600: F, t40607: F, t40611: F, t4362: F, t50634: F, t50643: F, t50673: F, t50681: F) -> (F, F) {
    let t61833 = t10886 * t808 * t18599;
    let t61837 = t1559 * t1544;
    let t61839 = t40834 * t854 * t61837;
    let t61852 = F::cast_from(0.90702367218671976884e-1_f64) * t50634 - F::cast_from(0.25410001404642664112e-4_f64) * t50643 - F::cast_from(0.50820002809285328225e-4_f64) * t61833 + F::cast_from(0.45351183609335988442e-1_f64) * t40594 + F::cast_from(0.10164000561857065645e-4_f64) * t40600 + t40607 - t40611 - F::cast_from(0.2032800112371413129e-4_f64) * t61839 + F::cast_from(0.50820002809285328224e-4_f64) * t50673 - F::cast_from(0.10841600599314203354e-2_f64) * t50681 + F::cast_from(0.85748036236139473945e-2_f64) * t4362 * t10770 * t18426 * t2723 * t2394 + F::cast_from(0.34299214494455789578e-2_f64) * t2745 * t2747 * t14676 * t18637;
    (t61837, t61852)
}
