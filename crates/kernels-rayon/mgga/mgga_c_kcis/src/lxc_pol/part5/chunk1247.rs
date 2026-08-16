//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1247/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1247(t1301: f64, t6888: f64, t1640: f64, t6896: f64, t446: f64, t6298: f64, t911: f64, t6884: f64, t1300: f64, t7570: f64, t1295: f64, t6294: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20856 = t6888 * t1301;
    let t20858 = t6896 * t1640;
    let t20859 = t446 * t20858;
    let t20861 = t911 * t6298;
    let t20863 = t911 * t6884;
    let t20865 = t1300 * t7570;
    let t20866 = t446 * t20865;
    let t20869 = t6294 * t1295;
    (t20856, t20859, t20861, t20863, t20866, t20869)
}
