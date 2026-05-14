//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1180/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1180<F: Float>(t34888: F, t1339: F, t3394: F, t6578: F, t6583: F, t31065: F, t10144: F, t1572: F, t4673: F, t10455: F, t4950: F, t10140: F, t10348: F, t8155: F, t31770: F, t6824: F) -> (F, F, F, F, F, F, F, F) {
    let t34889 = 0.76685851907841499352e0 * t34888;
    let t34890 = t1339 * t3394;
    let t34892 = t6583 * t34890 * t6578;
    let t34893 = 0.19171462976960374838e1 * t34892;
    let t34894 = 0.31952438294933958064e-1 * t31065;
    let t34897 = 0.95334639871601137784e0 * t1572 * t4673 * t10144;
    let t34900 = 0.95334639871601137784e0 * t4950 * t10455;
    let t34903 = 0.95334639871601137784e0 * t1572 * t4673 * t10140;
    let t34905 = 0.14300195980740170668e1 * t8155 * t10348;
    let t34910 = 0.95334639871601137784e0 * t6824 * t31770;
    (t34889, t34893, t34894, t34897, t34900, t34903, t34905, t34910)
}
