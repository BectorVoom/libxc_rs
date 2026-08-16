//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1055/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1055(t2098: f64, t7961: f64, t1851: f64, t8822: f64, t34102: f64, t576: f64, t112: f64, t34076: f64, t117397: f64, t120800: f64, t120803: f64, t120807: f64, t120809: f64, t120833: f64, t120849: f64, t16524: f64, t2039: f64, t27170: f64, t27254: f64, t31284: f64, t32308: f64, t33185: f64, t33195: f64, t5376: f64, t671: f64, t7056: f64, t7230: f64, t75795: f64, t8508: f64, t8717: f64, t94127: f64) -> (f64, f64, f64, f64) {
    let t124603 = t2098 * t7961;
    let t124609 = t1851 * t8822;
    let t124612 = t576 * t34102;
    let t124630 = t34076 * t112;
    let t124635 = t31284 + t8508 + 27.0_f64 * t120849 * t8717 + 27.0_f64 * t120833 * t8717 + 27.0_f64 * t75795 * t8717 + 27.0_f64 * t117397 * t5376 + 27.0_f64 * t27254 * t7056 + t33195 + 54.0_f64 * t16524 * t32308 + t120800 + t120803 + 27.0_f64 * t94127 * t2039 + 27.0_f64 * t7230 * t27170 + 0.135e2_f64 * t124630 * t671 + t120807 + 54.0_f64 * t33185 * t32308 + t120809;
    (t124603, t124609, t124612, t124635)
}
