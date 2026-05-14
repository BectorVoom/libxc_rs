//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1157/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1157<F: Float>(t10091: F, t31783: F, t12058: F, t4908: F, t1615: F, t3804: F, t1617: F, t10099: F, t10791: F, t12285: F, t7063: F, t972: F, t12288: F, t23723: F, t3622: F, t9375: F) -> (F, F, F, F, F, F, F) {
    let t36460 = 6.0 * t31783 * t10091;
    let t36462 = 4.0 * t4908 * t12058;
    let t36463 = t3804 * t1615;
    let t36465 = 2.0 * t36463 * t1617;
    let t36467 = 4.0 * t10099 * t10791;
    let t36470 = 12.0 * t7063 * t12285 * t972;
    let t36472 = 12.0 * t23723 * t12288;
    let t36474 = 2.0 * t9375 * t3622;
    (t36460, t36462, t36465, t36467, t36470, t36472, t36474)
}
