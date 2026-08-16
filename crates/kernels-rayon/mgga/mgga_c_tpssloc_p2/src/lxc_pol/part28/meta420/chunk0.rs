//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1594/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1594(t1995: f64, t9223: f64, t213: f64, t1999: f64, t1372: f64, t552: f64, t1307: f64, t6637: f64, t6888: f64, t3719: f64, t6968: f64, t117: f64, t547: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22881 = t552 * t1372;
    let t22882 = t22881 * t1307;
    let t22883 = t6637 * t22882;
    let t22884 = t6888 * t22883;
    let t22886 = t6968 * t3719;
    let t22887 = t6637 * t22886;
    let t22888 = t6888 * t22887;
    let t22891 = t547 * t67 * t117;
    (t22865, t22867, t22881, t22882, t22883, t22884, t22886, t22887, t22888, t22891)
}
