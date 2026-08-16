//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2195/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2195(t24996: f64, t97890: f64, t28860: f64, t6876: f64, t1307: f64, t6324: f64, t22574: f64, t26162: f64, t28835: f64, t28830: f64, t24995: f64, t8643: f64) -> (f64, f64, f64, f64, f64) {
    let t97892 = 12.0_f64 * t97890 * t24996;
    let t97893 = t6876 * t28860;
    let t97894 = t6324 * t1307;
    let t97897 = 6.0_f64 * t22574 * t26162 * t97894;
    let t97899 = 3.0_f64 * t6876 * t28835;
    let t97902 = t28830 * t1307;
    let t97905 = 12.0_f64 * t24995 * t8643 * t97902;
    (t97892, t97893, t97897, t97899, t97905)
}
