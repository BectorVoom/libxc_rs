//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3118/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3118(t18934: f64, t3411: f64, t1164: f64, t4882: f64, t51613: f64, t18274: f64, t3404: f64, t300: f64, t63709: f64, t63290: f64, t64475: f64, t64477: f64, t64479: f64, t64481: f64, t64485: f64, t64489: f64, t64492: f64, t64496: f64, t64499: f64) -> (f64, f64, f64, f64, f64) {
    let t64501 = 0.23392894490538584828e1_f64 * t3411 * t18934;
    let t64504 = 0.34631718211362927518e2_f64 * t1164 * t4882 * t51613;
    let t64507 = 0.6233709278045326953e3_f64 * t1164 * t18274 * t3404;
    let t64509 = 0.19751673498613801407e-1_f64 * t300 * t63709;
    let t64510 = t64475 - t64477 - t64479 - t64481 + t64485 + t64489 + t64492 - t64496 - t64499 + t64501 - t63290 - t64504 - t64507 + t64509;
    (t64501, t64504, t64507, t64509, t64510)
}
