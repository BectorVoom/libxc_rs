//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3233/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3233<F: Float>(t50852: F, t50856: F, t18562: F, t2516: F, t2496: F, t18305: F, t2258: F, t4401: F, t14325: F, t18306: F, t5825: F, t749: F) -> (F, F, F, F, F, F, F) {
    let t61292 = F::cast_from(0.10389515463408878255e3_f64) * t50852;
    let t61293 = F::cast_from(0.11393789434848516923e-2_f64) * t50856;
    let t61294 = t18562 * t2516;
    let t61295 = F::cast_from(0.5848223622634646207e0_f64) * t61294;
    let t61296 = t18562 * t2496;
    let t61297 = F::cast_from(0.17315859105681463759e2_f64) * t61296;
    let t61300 = F::new(12.0) * t4401 * t18305 * t2258;
    let t61302 = F::new(24.0) * t14325 * t18306;
    let t61303 = t749 * t5825;
    (t61292, t61293, t61295, t61297, t61300, t61302, t61303)
}
