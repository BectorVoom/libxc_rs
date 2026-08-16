//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 967/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk967(t1165: f64, t1567: f64, t3194: f64, t4210: f64, t3216: f64, t4360: f64, t1089: f64, t175: f64, t3037: f64, t3210: f64, t495: f64, t1008: f64, t4518: f64) -> (f64, f64, f64, f64) {
    let t15626 = t3194 * t1165 * t1567 * t4210;
    let t15628 = t3216 * t4360;
    let t15633 = t3210 * t1089 * t175 * t495 * t3037;
    let t15639 = t1008 * t4518;
    (t15626, t15628, t15633, t15639)
}
