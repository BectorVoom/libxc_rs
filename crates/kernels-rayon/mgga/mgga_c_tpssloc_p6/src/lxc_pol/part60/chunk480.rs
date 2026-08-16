//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 480/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk480(t5893: f64, t974: f64, t1044: f64, t248: f64, t5681: f64, t225: f64, t5848: f64, t68: f64, t369: f64, t1539: f64, t1616: f64, t3071: f64) -> (f64, f64, f64, f64, f64) {
    let t5894 = t974 * t5893;
    let t5900 = t248 * t1044 * t5681;
    let t5903 = t5848 * t225;
    let t5904 = t5903 * t68;
    let t5905 = t5904 * t369;
    let t5908 = t1616 * t1539;
    let t5909 = t3071 * t5908;
    (t5894, t5900, t5903, t5905, t5909)
}
