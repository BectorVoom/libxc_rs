//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 653/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk653(t12043: f64, t12073: f64, t12085: f64, t12086: f64, t12106: f64, t12131: f64, t12133: f64, t12145: f64, t1628: f64, t3745: f64, t10813: f64, t10815: f64, t10819: f64, t10823: f64, t10825: f64, t10830: f64, t10836: f64, t10839: f64, t10842: f64, t10845: f64, t833: f64, t9788: f64, t9798: f64, t9803: f64, t9809: f64) -> (f64, f64) {
    let t12148 = t12043 + t12073 + t12085 + t12086 + t12106 + t12131 + t12133 + t12145;
    let t12153 = t1628 * t3745;
    let t12156 = t10813 - t10815 - t10819 - t10823 - t10825 - t10830 + t10836 - t10839 + t10842 + t10845 + 0.38342925953920749677e0_f64 * t9788 - 0.76685851907841499354e0_f64 * t9798 + t9803 - t9809 + 0.30674340763136599741e1_f64 * t833 * t12153;
    (t12148, t12156)
}
