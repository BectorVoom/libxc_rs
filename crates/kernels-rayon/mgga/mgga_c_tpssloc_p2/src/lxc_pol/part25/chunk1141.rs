//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1141/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1141(t10003: f64, t23146: f64, t10009: f64, t25084: f64, t9629: f64, t9623: f64, t23127: f64, t2707: f64, t2690: f64, t6619: f64, t812: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81752 = t23146 * t10003;
    let t81754 = t23146 * t10009;
    let t81756 = t25084 * t9629;
    let t81758 = t23146 * t9623;
    let t81760 = t23127 * t2707;
    let t81763 = t812 * t6619 * t2690;
    let t81764 = t81763 * t849;
    (t81752, t81754, t81756, t81758, t81760, t81764)
}
