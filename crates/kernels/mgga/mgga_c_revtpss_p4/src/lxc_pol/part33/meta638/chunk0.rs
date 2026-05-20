//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2087/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2087<F: Float>(t101455: F, t116: F, t28042: F, t101451: F, t98141: F, t98148: F, t98161: F, t98165: F, t98200: F, t98218: F, t98220: F, t98224: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t101456 = F::new(2.0) / F::new(3.0) * t101455;
    let t101622 = t116 * t28042;
    let t101754 = F::new(22.0) / F::new(9.0) * t101451;
    let t102486 = F::cast_from(0.30488190661738479625e-3_f64) * t98141;
    let t102489 = F::cast_from(0.2168320119862840671e-2_f64) * t98148;
    let t102495 = F::cast_from(0.10164000561857065645e-4_f64) * t98161;
    let t102498 = F::cast_from(0.90702367218671976884e-1_f64) * t98165;
    let t102515 = F::cast_from(0.40656002247428262579e-4_f64) * t98200;
    let t102526 = F::cast_from(0.1219527626469539185e-2_f64) * t98218;
    let t102527 = F::cast_from(0.18071592998981862717e-4_f64) * t98220;
    let t102529 = F::cast_from(0.22675591804667994221e-1_f64) * t98224;
    (t101456, t101622, t101754, t102486, t102489, t102495, t102498, t102515, t102526, t102527, t102529)
}
