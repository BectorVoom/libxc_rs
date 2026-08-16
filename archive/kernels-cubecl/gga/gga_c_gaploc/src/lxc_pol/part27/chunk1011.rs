//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1011/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1011<F: Float>(t10813: F, t10815: F, t10819: F, t10823: F, t10825: F, t10830: F, t10836: F, t10839: F, t10842: F, t10845: F, t12153: F, t833: F, t9788: F, t9798: F, t9803: F, t9809: F) -> F {
    let t12156 = t10813 - t10815 - t10819 - t10823 - t10825 - t10830 + t10836 - t10839 + t10842 + t10845 + F::cast_from(0.38342925953920749677e0_f64) * t9788 - F::cast_from(0.76685851907841499354e0_f64) * t9798 + t9803 - t9809 + F::cast_from(0.30674340763136599741e1_f64) * t833 * t12153;
    t12156
}
