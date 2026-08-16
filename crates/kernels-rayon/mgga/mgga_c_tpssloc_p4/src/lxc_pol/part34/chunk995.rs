//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 995/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk995(t1995: f64, t9223: f64, t213: f64, t1999: f64, t117: f64, t547: f64, t67: f64, t6559: f64) -> (f64, f64, f64, f64) {
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22891 = t547 * t67 * t117;
    let t22892 = t6559 * t22891;
    (t22865, t22867, t22891, t22892)
}
