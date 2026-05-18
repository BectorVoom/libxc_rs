//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1339/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1339<F: Float>(t24215: F, t2801: F, t1052: F, t1960: F, t7817: F, t2208: F, t3459: F, t5559: F, t1381: F, t3362: F, t1383: F, t23555: F, t8443: F) -> (F, F, F, F, F) {
    let t33952 = F::new(4.0) * t24215 * t2801;
    let t33955 = F::new(2.0) * t1960 * t1052 * t7817;
    let t33958 = F::new(6.0) * t5559 * t3459 * t2208;
    let t33959 = t3362 * t1381;
    let t33961 = F::new(2.0) * t33959 * t1383;
    let t33963 = F::new(6.0) * t23555 * t8443;
    (t33952, t33955, t33958, t33961, t33963)
}
