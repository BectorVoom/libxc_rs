//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1056/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1056<F: Float>(t43490: F, t7427: F, t7573: F, t43598: F, t7572: F, t10930: F, t10931: F, t43494: F, t33331: F, t33332: F, t2660: F, t33576: F) -> (F, F, F, F, F) {
    let t44053 = t7427 * t7573 * t43490;
    let t44057 = F::cast_from(0.62115540045351614476e2_f64) * t7572 * t7573 * t43598;
    let t44060 = F::cast_from(0.38649669361552115674e3_f64) * t10930 * t10931 * t43494;
    let t44064 = F::cast_from(0.13803453343411469884e3_f64) * t33331 * t33332 * t43494;
    let t44065 = t33576 * t2660;
    (t44053, t44057, t44060, t44064, t44065)
}
