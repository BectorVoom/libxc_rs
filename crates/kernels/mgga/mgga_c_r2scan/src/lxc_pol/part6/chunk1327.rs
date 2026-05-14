//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1327/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1327<F: Float>(t19388: F, t19394: F, t19405: F, t23796: F, t23798: F, t23801: F, t23802: F, t23803: F, t23829: F, t23831: F, t23835: F, t23837: F, t23895: F, t23896: F, t23897: F, t23902: F) -> (F,) {
    let t25027 = -t23796 - t23798 - t19388 - t19394 - t23801 + t23802 - t23803 + t19405 + t23829 - t23831 + t23835 + t23837 + t23895 + t23896 - t23897 + t23902;
    (t25027,)
}
