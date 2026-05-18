//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 653/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk653<F: Float>(t12043: F, t12073: F, t12085: F, t12086: F, t12106: F, t12131: F, t12133: F, t12145: F, t1628: F, t3745: F, t10813: F, t10815: F, t10819: F, t10823: F, t10825: F, t10830: F, t10836: F, t10839: F, t10842: F, t10845: F, t833: F, t9788: F, t9798: F, t9803: F, t9809: F) -> (F, F) {
    let t12148 = t12043 + t12073 + t12085 + t12086 + t12106 + t12131 + t12133 + t12145;
    let t12153 = t1628 * t3745;
    let t12156 = t10813 - t10815 - t10819 - t10823 - t10825 - t10830 + t10836 - t10839 + t10842 + t10845 + F::new(0.38342925953920749677e0) * t9788 - F::new(0.76685851907841499354e0) * t9798 + t9803 - t9809 + F::new(0.30674340763136599741e1) * t833 * t12153;
    (t12148, t12156)
}
