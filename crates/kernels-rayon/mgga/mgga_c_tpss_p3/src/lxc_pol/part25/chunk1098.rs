//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1098/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1098(t1474: f64, t3949: f64, t4988: f64, t975: f64, t15076: f64, t366: f64, t2799: f64, t345: f64, t9080: f64, t948: f64, t1477: f64, t15117: f64, t15143: f64, t15147: f64, t15151: f64, t15155: f64, t15162: f64, t220: f64, t2782: f64, t2786: f64, t2798: f64, t368: f64, t3997: f64, t4008: f64, t5012: f64, t5021: f64, t5025: f64, t5029: f64, t9077: f64, t9094: f64, t9117: f64, t983: f64, t985: f64) -> f64 {
    let t15166 = t1474 * t3949;
    let t15176 = t975 * t4988;
    let t15179 = t366 * t15076;
    let t15186 = t2799 * t3949;
    let t15191 = t9080 * t948 * t345;
    let t15199 = t5012 * t948 * t983 * t985 + 4.0_f64 * t1477 * t15151 * t2782 - 2.0_f64 * t1477 * t15186 * t2798 + t15117 * t220 * t368 + 6.0_f64 * t15143 * t5021 * t9077 + 2.0_f64 * t15147 * t2782 * t2786 - t15147 * t2798 * t2799 - 6.0_f64 * t15155 * t5021 * t9094 + 2.0_f64 * t15162 * t983 * t985 + 2.0_f64 * t15166 * t983 * t985 + t15176 * t983 * t985 + t15179 * t983 * t985 + t15191 * t5021 * t9117 + 4.0_f64 * t2782 * t3997 * t5025 + 2.0_f64 * t2782 * t3997 * t5029 - 2.0_f64 * t2798 * t4008 * t5025 - t2798 * t4008 * t5029;
    t15199
}
