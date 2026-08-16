//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1341/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1341(t104122: f64, t24682: f64, t460: f64, t52: f64, t6144: f64, t18356: f64, t24729: f64, t27614: f64, t4997: f64, t1730: f64, t27603: f64, t27598: f64, t5001: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104239 = t24682 * t104122 * t460;
    let t104280 = t52 * t6144;
    let t104282 = t24682 * t104280 * t460;
    let t104294 = t24729 * t18356;
    let t104296 = t27614 * t4997;
    let t104300 = t1730 * t27603;
    let t104303 = t5001 * t27598;
    (t104239, t104280, t104282, t104294, t104296, t104300, t104303)
}
