//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1062/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1062(t1505: f64, t5895: f64, t2016: f64, t4188: f64, t12321: f64, t41: f64, t4291: f64, t5747: f64, t2033: f64, t4121: f64, t492: f64, t6015: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17308 = t5895 * t1505;
    let t17311 = t2016 * t4188;
    let t17382 = t41 * t12321;
    let t17391 = t5747 * t4291;
    let t17396 = t2033 * t4121;
    let t17412 = t6015 * t492;
    (t17308, t17311, t17382, t17391, t17396, t17412)
}
