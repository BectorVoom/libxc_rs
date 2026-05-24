//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 676/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk676<F: Float>(t1781: F, t657: F, t164: F, t1774: F, t25: F, t5005: F, t1736: F, t4953: F, t4956: F, t633: F, t630: F, t1704: F, t4907: F) -> (F, F, F, F, F, F, F) {
    let t10871 = t1781 * t1781;
    let t10872 = F::new(1.0) / t10871;
    let t10873 = t657 * t10872;
    let t10879 = t164 * t1774;
    let t10886 = t25 * t5005;
    let t10902 = F::new(1.0) / t4953 / t1736;
    let t10906 = F::new(1.0) / t4956 / t633;
    let t10913 = F::new(1.0) / t4953 / t630;
    let t10924 = F::new(1.0) / t4907 / t1704;
    (t10873, t10879, t10886, t10902, t10906, t10913, t10924)
}
