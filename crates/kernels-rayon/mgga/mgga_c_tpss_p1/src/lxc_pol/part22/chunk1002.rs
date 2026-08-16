//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1002/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1002(t3638: f64, t8313: f64, t236: f64, t339: f64, t8276: f64, t2161: f64, t8279: f64, t3628: f64, t3629: f64, t2163: f64, t2175: f64, t3676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10777 = 7.0_f64 / 576.0_f64 * t8313 * t3638;
    let t10779 = t339 * t8276 * t236;
    let t10780 = t8279 * t2161;
    let t10782 = t3628 * t3629 * t10780;
    let t10786 = t3628 * t3629 * t2163;
    let t10790 = t2175 * t3676 * t2163;
    (t10777, t10779, t10780, t10782, t10786, t10790)
}
