//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 743/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk743(t15625: f64, t359: f64, t356: f64, t89: f64, t1597: f64, t4441: f64, t63: f64, t7857: f64, t3099: f64, t930: f64, t374: f64, t11375: f64, t938: f64) -> (f64, f64, f64, f64, f64) {
    let t15626 = t359 * t15625;
    let t15628 = t89 * t356 * t15626;
    let t15630 = t4441 * t1597;
    let t15631 = t15630 * t63;
    let t15632 = t7857 * t15631;
    let t15635 = t930 * t3099;
    let t15636 = t374 * t15635;
    let t15639 = t11375 * t938;
    (t15628, t15630, t15632, t15636, t15639)
}
