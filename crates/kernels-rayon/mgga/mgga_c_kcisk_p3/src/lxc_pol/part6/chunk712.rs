//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 712/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk712(t12671: f64, t3260: f64, t3232: f64, t981: f64, t1036: f64, t1039: f64, t3139: f64, t3241: f64, t212: f64, t916: f64, t211: f64, t210: f64) -> (f64, f64, f64, f64) {
    let t12672 = t12671 * t3260;
    let t12674 = t3232 * t981;
    let t12675 = t12674 * t1036;
    let t12677 = t1039 * t3139;
    let t12678 = t3241 * t12677;
    let t12680 = t212 * t916;
    let t12681 = 1.0_f64 / t12680;
    let t12682 = t211 * t12681;
    let t12683 = t210 * t12682;
    (t12672, t12675, t12678, t12683)
}
