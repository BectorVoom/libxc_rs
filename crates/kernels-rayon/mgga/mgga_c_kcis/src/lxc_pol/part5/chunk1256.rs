//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1256/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1256(t1468: f64, t20989: f64, t1464: f64, t1889: f64, t5676: f64, t15887: f64, t4160: f64, t5880: f64, t12281: f64, t2001: f64, t833: f64, t5440: f64) -> (f64, f64, f64, f64, f64) {
    let t20990 = t1468 * t20989;
    let t20991 = t1464 * t20990;
    let t20994 = t1889 * t5676;
    let t20995 = t15887 * t20994;
    let t20996 = t4160 * t20995;
    let t20998 = t1889 * t5880;
    let t20999 = t12281 * t20998;
    let t21000 = t4160 * t20999;
    let t21002 = t2001 * t833;
    let t21003 = t5440 * t21002;
    (t20991, t20996, t21000, t21002, t21003)
}
