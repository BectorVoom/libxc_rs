//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1036/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1036<F: Float>(t23343: F, t23382: F, t23409: F, t23761: F, t23789: F, t23816: F, t23839: F, t23865: F, t1791: F, t10409: F, t8481: F, t4811: F, t8883: F, t8886: F, t8875: F, t8879: F) -> (F, F, F, F, F, F, F) {
    let t23868 = t23343 + t23382 + t23409 + t23761 + t23789 + t23816 + t23839 + t23865;
    let t23869 = t23868 * t1791;
    let t23872 = t10409 * t8481;
    let t23874 = t4811 * t8883;
    let t23876 = t4811 * t8886;
    let t23878 = t4811 * t8875;
    let t23880 = t4811 * t8879;
    (t23868, t23869, t23872, t23874, t23876, t23878, t23880)
}
