//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 903/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk903<F: Float>(t13126: F, t2194: F, t1445: F, t2087: F, t43240: F, t13161: F, t5782: F, t13125: F, t4614: F, t813: F, t13149: F, t2464: F, t825: F, t10930: F, t10931: F, t43490: F) -> (F, F, F, F, F, F) {
    let t44030 = t2194 * t13126;
    let t44038 = 0.62115540045351614476e2 * t2087 * t1445 * t43240;
    let t44040 = 0.62115540045351614476e2 * t5782 * t13161;
    let t44042 = t813 * t4614 * t13125;
    let t44045 = t825 * t2464 * t13149;
    let t44046 = 0.63904876589867916128e-1 * t44045;
    let t44048 = t10930 * t10931 * t43490;
    (t44030, t44038, t44040, t44042, t44046, t44048)
}
