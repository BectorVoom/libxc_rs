//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1044/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1044<F: Float>(t1899: F, t24001: F, t1873: F, t1869: F, t15851: F, t2473: F, t1799: F, t17077: F, t17087: F, t23976: F, t23978: F, t23983: F, t23988: F, t23992: F, t23996: F, t23999: F) -> (F, F, F) {
    let t24002 = t1899 * t24001;
    let t24003 = t1873 * t24002;
    let t24004 = t1869 * t24003;
    let t24006 = t15851 * t2473;
    let t24007 = t1799 * t24006;
    let t24009 = 0.88437037037037037033e-2 * t23976 + t17077 + 0.22109259259259259259e-2 * t23978 - 0.1492375e-1 * t23983 + t17087 + 0.66327777777777777776e-2 * t23988 - 0.55273148148148148147e-2 * t23992 + 0.88437037037037037033e-2 * t23996 - 0.33163888888888888888e-2 * t23999 - 0.33163888888888888888e-2 * t24004 + 0.33163888888888888888e-2 * t24007;
    (t24004, t24007, t24009)
}
