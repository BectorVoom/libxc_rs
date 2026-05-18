//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 623/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk623<F: Float>(t1856: F, t8514: F, t1842: F, t8518: F, t1659: F, t8510: F, t1835: F, t165: F, t173: F, t5122: F, t5125: F, t5128: F, t5129: F, t5135: F, t5168: F, t7715: F) -> (F, F, F, F, F, F) {
    let t8620 = t1856 * t8514;
    let t8623 = t1842 * t8518;
    let t8626 = t1659 * t8510;
    let t8629 = t1856 * t8518;
    let t8632 = t1835 * t8510;
    let t8637 = t5122 - t5125 - t5128 - F::new(0.10082625e-4) * t173 * t8620 + F::new(0.7925e-3) * t165 * t8623 - F::new(0.52833333333333333333e-3) * t165 * t8626 + F::new(0.50413125e-5) * t173 * t8629 - F::new(0.672175e-5) * t173 * t8632 - t5129 + t5135 + F::new(0.15538616723388920628e-3) * t5168 * t7715;
    (t8620, t8623, t8626, t8629, t8632, t8637)
}
