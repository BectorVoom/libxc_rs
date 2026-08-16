//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2262/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2262(t3375: f64, t6063: f64, t18893: f64, t3359: f64, t11285: f64, t6084: f64, t18785: f64, t3403: f64, t18834: f64, t3315: f64, t1147: f64, t18710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63454 = t6063 * t3375;
    let t63502 = t18893 * t3359;
    let t63519 = t6084 * t11285;
    let t63533 = t18785 * t3403;
    let t63588 = t18834 * t3315;
    let t63597 = t18710 * t1147;
    (t63454, t63502, t63519, t63533, t63588, t63597)
}
