//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1115/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1115<F: Float>(t2786: F, t5737: F, t5802: F, t1084: F, t5776: F, t1083: F, t17577: F, t17579: F, t5585: F, t7411: F, t1096: F, t17388: F, t17616: F, t21220: F, t21223: F, t21225: F, t21226: F, t21229: F, t21233: F, t21236: F, t2801: F, t2820: F, t5830: F, t5831: F, t5883: F, t704: F, t723: F, t7486: F) -> (F, F, F, F, F) {
    let t21239 = 0.57895126195293126241e3 * t5802 * t2786 * t5737;
    let t21251 = 24.0 * t5776 * t1084 * t5737;
    let t21255 = 0.24955700379505800916e5 * t17577 * t1083 * t17579 * t5737;
    let t21257 = 0.48245938496077605201e2 * t7411 * t5585;
    let t21258 = -t21220 - t21223 - t21225 + 0.17544670867903938621e1 * t21226 * t723 + 3.0 * t21229 * t704 - t21233 - t21236 - t21239 - 24.0 * t5830 * t1096 * t5831 - 6.0 * t7486 * t5883 - 6.0 * t17388 * t2801 + 0.96491876992155210402e2 * t17616 * t2820 + t21251 - t21255 - t21257;
    (t21239, t21251, t21255, t21257, t21258)
}
