//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1022/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1022<F: Float>(t1248: F, t22506: F, t4893: F, t1720: F, t22592: F, t10999: F, t8510: F, t4889: F, t8518: F, t10937: F, t11040: F, t17379: F, t17382: F, t17453: F, t17454: F, t23460: F, t23463: F, t23466: F, t23469: F, t23472: F, t23475: F, t23478: F, t23481: F, t23484: F, t23487: F, t23490: F) -> (F, F, F, F, F) {
    let t23599 = t1248 * t4893 * t22506;
    let t23602 = t1248 * t1720 * t22592;
    let t23606 = t1248 * t10999 * t8510;
    let t23609 = t1248 * t4889 * t8518;
    let t23625 = -t11040 - 4.0 / 27.0 * t10937 - 8.0 / 27.0 * t17382 + t17453 - t17454 - 4.0 / 9.0 * t17379 + 2.0 / 27.0 * t23460 - 10.0 / 27.0 * t23463 + 4.0 / 3.0 * t23466 + 8.0 / 9.0 * t23469 - 2.0 / 9.0 * t23472 - 2.0 * t23475 - 8.0 / 3.0 * t23478 + t23481 / 9.0 - 2.0 / 9.0 * t23484 + 2.0 / 3.0 * t23487 - t23490 / 3.0;
    (t23599, t23602, t23606, t23609, t23625)
}
