//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1409/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1409<F: Float>(t3394: F, t493: F, t6576: F, t6578: F, t1339: F, t6583: F, t31065: F, t10144: F, t1572: F, t4673: F, t1436: F, t31040: F, t31044: F, t31046: F, t31050: F, t31053: F, t31056: F, t31068: F, t34874: F, t34877: F, t34879: F, t34881: F, t34882: F, t590: F) -> F {
    let t34886 = t493 * t3394;
    let t34888 = t6576 * t34886 * t6578;
    let t34889 = F::cast_from(0.76685851907841499352e0_f64) * t34888;
    let t34890 = t1339 * t3394;
    let t34892 = t6583 * t34890 * t6578;
    let t34893 = F::cast_from(0.19171462976960374838e1_f64) * t34892;
    let t34894 = F::cast_from(0.31952438294933958064e-1_f64) * t31065;
    let t34897 = F::cast_from(0.95334639871601137784e0_f64) * t1572 * t4673 * t10144;
    let t34898 = t31040 + t31044 - t31046 - t31050 + t31053 - t31056 - t34874 + t34877 + t34879 + t34881 - F::cast_from(0.1022478025437886658e1_f64) * t1436 * t34882 * t590 + t34889 - t34893 + t34894 - t31068 + t34897;
    t34898
}
