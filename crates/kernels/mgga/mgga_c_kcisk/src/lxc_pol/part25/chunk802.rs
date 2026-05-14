//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 802/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk802<F: Float>(t4989: F, t5002: F, t164: F, t1786: F, t1773: F, t25: F, t5039: F, t1781: F, t1774: F) -> (F, F, F, F, F) {
    let t10863 = t4989 * t5002;
    let t10865 = t164 * t1786;
    let t10866 = t1773 * t10865;
    let t10868 = t25 * t5039;
    let t10869 = t1773 * t10868;
    let t10871 = t1781 * t1781;
    let t10872 = 1.0 / t10871;
    let t10879 = t164 * t1774;
    (t10863, t10866, t10869, t10872, t10879)
}
