//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1341/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1341<F: Float>(t34878: F, t10409: F, t1441: F, t3394: F, t493: F, t6576: F, t6578: F, t1339: F, t6583: F, t31065: F, t10144: F, t1572: F, t4673: F) -> (F, F, F, F, F, F) {
    let t34879 = F::new(0.51123901271894332902e0) * t34878;
    let t34880 = t1441 * t10409;
    let t34881 = F::new(0.1022478025437886658e1) * t34880;
    let t34886 = t493 * t3394;
    let t34888 = t6576 * t34886 * t6578;
    let t34889 = F::new(0.76685851907841499352e0) * t34888;
    let t34890 = t1339 * t3394;
    let t34892 = t6583 * t34890 * t6578;
    let t34893 = F::new(0.19171462976960374838e1) * t34892;
    let t34894 = F::new(0.31952438294933958064e-1) * t31065;
    let t34897 = F::new(0.95334639871601137784e0) * t1572 * t4673 * t10144;
    (t34879, t34881, t34889, t34893, t34894, t34897)
}
