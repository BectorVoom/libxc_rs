//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1098/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1098<F: Float>(t1474: F, t3949: F, t4988: F, t975: F, t15076: F, t366: F, t2799: F, t345: F, t9080: F, t948: F, t1477: F, t15117: F, t15143: F, t15147: F, t15151: F, t15155: F, t15162: F, t220: F, t2782: F, t2786: F, t2798: F, t368: F, t3997: F, t4008: F, t5012: F, t5021: F, t5025: F, t5029: F, t9077: F, t9094: F, t9117: F, t983: F, t985: F) -> F {
    let t15166 = t1474 * t3949;
    let t15176 = t975 * t4988;
    let t15179 = t366 * t15076;
    let t15186 = t2799 * t3949;
    let t15191 = t9080 * t948 * t345;
    let t15199 = t5012 * t948 * t983 * t985 + F::cast_from(4.0_f64) * t1477 * t15151 * t2782 - F::cast_from(2.0_f64) * t1477 * t15186 * t2798 + t15117 * t220 * t368 + F::cast_from(6.0_f64) * t15143 * t5021 * t9077 + F::cast_from(2.0_f64) * t15147 * t2782 * t2786 - t15147 * t2798 * t2799 - F::cast_from(6.0_f64) * t15155 * t5021 * t9094 + F::cast_from(2.0_f64) * t15162 * t983 * t985 + F::cast_from(2.0_f64) * t15166 * t983 * t985 + t15176 * t983 * t985 + t15179 * t983 * t985 + t15191 * t5021 * t9117 + F::cast_from(4.0_f64) * t2782 * t3997 * t5025 + F::cast_from(2.0_f64) * t2782 * t3997 * t5029 - F::cast_from(2.0_f64) * t2798 * t4008 * t5025 - t2798 * t4008 * t5029;
    t15199
}
