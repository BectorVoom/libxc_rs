//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1043/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1043<F: Float>(t1966: F, t43842: F, t590: F, t1890: F, t43107: F, t10948: F, t11016: F, t13012: F, t2087: F, t4614: F, t3267: F, t8634: F) -> (F, F, F, F, F) {
    let t43844 = t1966 * t43842 * t590;
    let t43849 = F::cast_from(0.25561950635947166451e1_f64) * t1966 * t1890 * t43107 * t590;
    let t43854 = t10948 * t11016;
    let t43858 = F::cast_from(0.92023022289409799224e1_f64) * t2087 * t4614 * t13012;
    let t43861 = F::cast_from(0.35750489951850426669e0_f64) * t3267 * t8634;
    (t43844, t43849, t43854, t43858, t43861)
}
