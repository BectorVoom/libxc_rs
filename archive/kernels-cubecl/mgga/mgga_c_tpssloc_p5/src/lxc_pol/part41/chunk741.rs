//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 741/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk741<F: Float>(t5045: F, t974: F, t1174: F, t1198: F, t1213: F, t1218: F, t1227: F, t1232: F, t1748: F, t3490: F, t3524: F, t3542: F, t3543: F, t3547: F, t3549: F, t3573: F, t4889: F, t5014: F, t5019: F, t5024: F, t5030: F, t5033: F, t5036: F, t5041: F) -> (F, F) {
    let t5046 = t974 * t5045;
    let t5051 = t1213 * t5014 / F::cast_from(3072.0_f64) - t5019 * t1218 / F::cast_from(576.0_f64) + t5024 * t1232 / F::cast_from(864.0_f64) - t3490 * t1748 / F::cast_from(4608.0_f64) - t1227 * t5030 / F::cast_from(4608.0_f64) + t1174 * t5033 / F::cast_from(216.0_f64) - t5036 / F::cast_from(108.0_f64) - t3524 / F::cast_from(6912.0_f64) + t3573 / F::cast_from(4608.0_f64) - t5041 / F::cast_from(864.0_f64) + t4889 * t1198 / F::cast_from(108.0_f64) - t1174 * t5046 / F::cast_from(288.0_f64) - t3549 / F::cast_from(864.0_f64) - t3542 + t3543 / F::cast_from(4608.0_f64) - t3547;
    (t5046, t5051)
}
