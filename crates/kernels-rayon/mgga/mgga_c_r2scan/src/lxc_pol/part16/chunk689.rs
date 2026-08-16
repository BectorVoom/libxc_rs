//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 689/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk689(t182: f64, t518: f64, t190: f64, t625: f64, t1827: f64, t732: f64, t1842: f64, t1810: f64, t1838: f64, t1826: f64, t1830: f64, t234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5332 = t518 * t182;
    let t5335 = 0.55403703703703703703e-1_f64 * t625 * t5332 * t190;
    let t5338 = t732 * t1827;
    let t5340 = t732 * t1842;
    let t5344 = t732 * t1810;
    let t5346 = t732 * t1838;
    let t5348 = t1826 * t1830;
    let t5350 = 0.35089341735807877242e1_f64 * t234 * t5348;
    (t5335, t5338, t5340, t5344, t5346, t5350)
}
