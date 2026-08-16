//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2312/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2312(t40804: f64, t40806: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t46311: f64, t67214: f64, t67215: f64, t12939: f64, t16716: f64, t3966: f64) -> (f64, f64, f64, f64) {
    let t67216 = 0.32530743900905219526e-1_f64 * t40804;
    let t67217 = 0.48159733137676571078e0_f64 * t40806;
    let t67218 = -t46311 + t67214 + t40790 + t40793 + t67215 + t40797 + t40799 + t40801 - t40803 - t67216 + t67217;
    let t67226 = 72.0_f64 * t12939 * t16716 * t3966;
    (t67216, t67217, t67218, t67226)
}
