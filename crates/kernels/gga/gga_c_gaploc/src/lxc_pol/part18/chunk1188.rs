//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1188/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1188<F: Float>(t2902: F, t6553: F, t24295: F, t2595: F, t11135: F, t5559: F, t841: F, t24282: F, t921: F, t7324: F, t8859: F, t19933: F, t8054: F, t2592: F, t8854: F, t10283: F, t1651: F) -> (F, F, F, F, F, F, F, F) {
    let t33968 = 2.0 * t6553 * t2902;
    let t33970 = 4.0 * t24295 * t2595;
    let t33973 = 12.0 * t5559 * t11135 * t841;
    let t33974 = t24282 * t921;
    let t33977 = 4.0 * t7324 * t8859;
    let t33979 = 6.0 * t19933 * t8054;
    let t33980 = t2592 * t8854;
    let t33981 = t10283 * t1651;
    (t33968, t33970, t33973, t33974, t33977, t33979, t33980, t33981)
}
