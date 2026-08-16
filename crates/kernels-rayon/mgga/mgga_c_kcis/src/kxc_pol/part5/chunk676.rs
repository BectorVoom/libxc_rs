//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 676/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk676(t1045: f64, t4848: f64, t1035: f64, t4670: f64, t102: f64, t2474: f64) -> (f64, f64, f64) {
    let t4849 = t4848 * t1045;
    let t4852 = t1035 * t4670;
    let t4858 = t102 * t2474;
    (t4849, t4852, t4858)
}
