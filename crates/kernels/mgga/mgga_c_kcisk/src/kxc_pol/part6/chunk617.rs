//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 617/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk617<F: Float>(t4704: F, t8550: F, t4716: F, t8504: F, t1653: F, t8522: F, t4726: F, t8510: F, t26: F, t1659: F, t8514: F, t8518: F) -> (F, F, F, F, F, F, F, F) {
    let t8552 = F::new(2.0) * t4704 * t8550;
    let t8559 = t4716 * t8504;
    let t8561 = t1653 * t8522;
    let t8564 = t4726 * t8510;
    let t8565 = t26 * t8564;
    let t8567 = t1659 * t8514;
    let t8568 = t26 * t8567;
    let t8570 = t1659 * t8518;
    (t8552, t8559, t8561, t8564, t8565, t8567, t8568, t8570)
}
