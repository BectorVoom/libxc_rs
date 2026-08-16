//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1324/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1324(t18246: f64, t44470: f64, t63863: f64, t10897: f64, t33: f64, t1659: f64, t3387: f64, t19619: f64, t5705: f64, t3234: f64, t13220: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65002 = t18246 * t44470;
    let t65013 = t18246 * t63863;
    let t65030 = t33 * t10897;
    let t65052 = t1659 * t3387;
    let t65056 = t5705 * t19619;
    let t65060 = t1659 * t3234;
    let t65067 = t94 * t13220;
    (t65002, t65013, t65030, t65052, t65056, t65060, t65067)
}
