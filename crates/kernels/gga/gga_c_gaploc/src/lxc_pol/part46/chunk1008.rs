//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1008/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1008<F: Float>(t13161: F, t5782: F, t13125: F, t4614: F, t813: F, t13149: F, t2464: F, t825: F, t10930: F, t10931: F, t43490: F, t24968: F, t9958: F) -> (F, F, F, F, F) {
    let t44040 = F::new(0.62115540045351614476e2) * t5782 * t13161;
    let t44042 = t813 * t4614 * t13125;
    let t44045 = t825 * t2464 * t13149;
    let t44046 = F::new(0.63904876589867916128e-1) * t44045;
    let t44048 = t10930 * t10931 * t43490;
    let t44051 = F::new(0.42900587942220512003e1) * t24968 * t9958;
    (t44040, t44042, t44046, t44048, t44051)
}
