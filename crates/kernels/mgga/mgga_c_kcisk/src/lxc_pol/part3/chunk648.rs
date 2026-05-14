//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 648/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk648<F: Float>(t25: F, t5039: F, t1773: F, t1781: F, t657: F, t1785: F, t5032: F, t1310: F, t164: F, t1774: F, t1777: F, t4998: F, t5025: F, t5005: F, t5008: F, t1744: F, t4928: F) -> (F, F, F, F, F, F, F, F) {
    let t10868 = t25 * t5039;
    let t10869 = t1773 * t10868;
    let t10871 = t1781 * t1781;
    let t10872 = 1.0 / t10871;
    let t10873 = t657 * t10872;
    let t10874 = t5032 * t1785;
    let t10875 = t10873 * t10874;
    let t10876 = t1310 * t10875;
    let t10879 = t164 * t1774;
    let t10880 = t10879 * t1777;
    let t10881 = t1773 * t10880;
    let t10883 = t4998 * t5025;
    let t10884 = t1773 * t10883;
    let t10886 = t25 * t5005;
    let t10887 = t10886 * t5008;
    let t10888 = t1773 * t10887;
    let t10892 = t4928 * t1744;
    (t10869, t10876, t10879, t10881, t10884, t10886, t10888, t10892)
}
