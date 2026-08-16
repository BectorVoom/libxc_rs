//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1182/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1182(t18670: f64, t5489: f64, t1791: f64, t18351: f64, t5492: f64, t5791: f64, t1844: f64, t507: f64, t3205: f64) -> (f64, f64, f64, f64, f64) {
    let t18671 = t18670 * t5489;
    let t18673 = t1791 * t18351;
    let t18676 = t5492 * t5791;
    let t18686 = t507 * t1844;
    let t18690 = t1844 * t3205;
    (t18671, t18673, t18676, t18686, t18690)
}
