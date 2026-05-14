//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 831/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk831<F: Float>(t1782: F, t7233: F, t1849: F, t569: F, t1310: F, t1764: F, t3934: F, t654: F, t164: F, t1786: F, t1773: F, t1781: F, t1774: F) -> (F, F, F, F, F, F, F) {
    let t10802 = t7233 * t1782;
    let t10831 = 1.0 / t569 / t1849;
    let t10832 = t1310 * t10831;
    let t10856 = t1764 * t654 * t3934;
    let t10865 = t164 * t1786;
    let t10866 = t1773 * t10865;
    let t10871 = t1781 * t1781;
    let t10872 = 1.0 / t10871;
    let t10879 = t164 * t1774;
    (t10802, t10831, t10832, t10856, t10866, t10872, t10879)
}
