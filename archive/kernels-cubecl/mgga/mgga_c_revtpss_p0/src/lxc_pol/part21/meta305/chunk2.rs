//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1563/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1563<F: Float>(t10722: F, t857: F, t10673: F, t10676: F, t10678: F, t10682: F, t10687: F, t10692: F, t10693: F, t10700: F, t10706: F, t10711: F, t10713: F, t10717: F, t10719: F, t851: F) -> (F, F) {
    let t10723 = t10722 * t857;
    let t10725 = t10673 - F::cast_from(0.42874018118069736972e-3_f64) * t10676 - F::cast_from(0.91464571985215438873e-3_f64) * t10678 + F::cast_from(0.85748036236139473944e-4_f64) * t10682 - t10687 + t10692 - F::cast_from(0.60023625365297631762e-1_f64) * t10693 - F::cast_from(0.25724410870841842183e-1_f64) * t851 * t10700 + F::cast_from(0.76230004213927992338e-3_f64) * t10706 + F::cast_from(0.21437009059034868486e-4_f64) * t10711 + F::cast_from(0.12004725073059526352e-1_f64) * t10713 + F::cast_from(0.16262400898971305032e-2_f64) * t10717 - F::cast_from(0.22866142996303859718e-3_f64) * t10719 - F::cast_from(0.68026775414003982663e-1_f64) * t10723;
    (t10723, t10725)
}
