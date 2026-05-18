//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1097/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1097<F: Float>(t13802: F, t949: F, t2986: F, t3031: F, t4758: F, t4764: F, t10974: F, t4763: F, t1692: F, t9630: F, t3006: F, t9634: F) -> (F, F, F, F, F) {
    let t13803 = t13802 * t949;
    let t13805 = F::new(0.32163648644302209644e2) * t2986 * t13803;
    let t13806 = t3031 * t4758;
    let t13807 = t13806 * t4764;
    let t13812 = t4763 * t10974;
    let t13817 = t9630 * t1692;
    let t13818 = t9634 * t3006;
    (t13805, t13807, t13812, t13817, t13818)
}
