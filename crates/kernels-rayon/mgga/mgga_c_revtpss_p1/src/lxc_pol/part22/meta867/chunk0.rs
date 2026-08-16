//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3023/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3023(t14325: f64, t14370: f64, t14322: f64, t2626: f64, t4398: f64, t9425: f64, t10555: f64, t14613: f64, t10565: f64, t1532: f64, t9419: f64, t162: f64, t40188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50880 = t14325 * t14370;
    let t50883 = t14322 * t2626;
    let t50888 = t4398 * t9425;
    let t50890 = t14613 * t10555;
    let t50892 = t1532 * t10565;
    let t50893 = t4398 * t9419;
    let t50895 = t40188 * t162;
    (t50880, t50883, t50888, t50890, t50892, t50893, t50895)
}
