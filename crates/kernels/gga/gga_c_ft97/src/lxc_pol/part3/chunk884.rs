//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 884/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk884<F: Float>(t19782: F, t312: F, t19329: F, t19334: F, t19345: F, t19379: F, t19383: F, t19391: F, t19431: F, t19436: F, t19810: F, t19863: F, t19885: F, t332: F, t113: F, t18795: F, t18799: F, t18802: F, t18804: F, t18809: F, t18812: F, t18946: F, t18953: F, t2904: F, t4322: F, t4391: F, t4395: F, t5: F, t5483: F, t889: F) -> (F,) {
    let t19886 = t19782 * t312;
    let t19898 = 2.0 * t19886 - 2.0 * t19334 - 4.0 * t19345 + 8.0 * t19383 - 4.0 * t19329 + 4.0 * t19810 - 12.0 * t19431 + 8.0 * t19436 - 2.0 * t19391 + 4.0 * t19379 - 2.0 * t19863;
    let t19899 = t19885 + t19898;
    let t19900 = t19899 * t332;
    let t19904 = t889 * t18795 / 2.0 + t889 * t18799 / 2.0 - t889 * t18802 + t889 * t18804 / 4.0 + t2904 * t5483 / 2.0 + t889 * t18809 / 2.0 + t889 * t18812 / 4.0 + t889 * t18946 / 4.0 + t4322 * t4391 / 2.0 - t4322 * t4395 + t889 * t18953 / 4.0 + t5 * t19900 * t113 / 4.0;
    (t19904,)
}
