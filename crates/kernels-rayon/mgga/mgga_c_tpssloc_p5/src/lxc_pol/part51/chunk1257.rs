//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1257/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1257(t225: f64, t26229: f64, t1324: f64, t254: f64, t22573: f64, t7684: f64, t6875: f64, t8944: f64, t111: f64, t26966: f64, t2094: f64, t40611: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91491 = t26229 * t225;
    let t91505 = t1324 * t254;
    let t91655 = t7684 * t22573;
    let t91669 = t6875 * t8944;
    let t92090 = t26966 * t111;
    let t92169 = t2094 * t40611;
    (t91491, t91505, t91655, t91669, t92090, t92169)
}
