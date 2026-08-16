//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1457/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1457(t1227: f64, t18975: f64, t4997: f64, t5019: f64, t4993: f64, t5005: f64, t1202: f64, t6164: f64, t5024: f64, t11692: f64, t11792: f64, t11821: f64, t15671: f64, t15691: f64, t15699: f64, t15740: f64, t18955: f64, t18959: f64, t18965: f64, t18969: f64, t18972: f64, t3577: f64, t488: f64, t4950: f64) -> f64 {
    let t18976 = t1227 * t18975;
    let t18978 = t5019 * t4997;
    let t18980 = t5005 * t4993;
    let t18982 = t1202 * t6164;
    let t18987 = t5024 * t4993;
    let t18989 = -5.0_f64 / 5184.0_f64 * t1227 * t18955 - t1227 * t18959 / 2304.0_f64 - t15740 * t4950 / 2304.0_f64 + t11692 * t18965 / 4608.0_f64 - t3577 * t18969 / 4608.0_f64 + t15671 + t18972 / 2304.0_f64 + 5.0_f64 / 20736.0_f64 * t18976 - t18978 / 432.0_f64 - t18980 / 3456.0_f64 + 19.0_f64 / 1728.0_f64 * t18982 * t488 + t11792 / 20736.0_f64 - t11821 / 13824.0_f64 + t18987 / 648.0_f64 - t15691 + t15699;
    t18989
}
