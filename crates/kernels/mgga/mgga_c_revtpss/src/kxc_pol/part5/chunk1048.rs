//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1048/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1048<F: Float>(t3566: F, t488: F, t1276: F, t1774: F, t1209: F, t1811: F, t1269: F, t1770: F, t3555: F, t460: F, t5412: F, t17306: F, t487: F, t5219: F, t5216: F, t1204: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    let t17986 = t1209 * t488;
    let t17995 = t3566 * t1811;
    let t18005 = t1770 * t1269;
    let t18037 = t3555 * t1811;
    let t18054 = t460 * t5412;
    let t18059 = t17306 * t487;
    let t18062 = t5219 * t1269;
    let t18065 = t5216 * t487;
    let t18087 = t1204 * t1811;
    (t17973, t17974, t17986, t17995, t18005, t18037, t18054, t18059, t18062, t18065, t18087)
}
