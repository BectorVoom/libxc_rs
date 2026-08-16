//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 756/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk756(t2844: f64, t359: f64, t4547: f64, t3210: f64, t4554: f64, t1754: f64, t3255: f64, t3262: f64, t347: f64, t1646: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4555 = t359 * t2844;
    let t4556 = t4555 * t4547;
    let t4557 = t3210 * t4556;
    let t4558 = t4554 * t4557;
    let t4563 = t3255 * t1754;
    let t4565 = t3262 * t347;
    let t4566 = t2844 * t1646;
    (t4555, t4556, t4557, t4558, t4563, t4565, t4566)
}
