//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1400/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1400<F: Float>(t10310: F, t780: F, t113: F, t32896: F, t2147: F, t2148: F, t20539: F, t20542: F, t20552: F, t20561: F, t25460: F, t25483: F, t25488: F, t29777: F, t29781: F, t29785: F, t29788: F, t29798: F) -> (F,) {
    let t33941 = t10310 * t780;
    let t33944 = t32896 * t113;
    let t33946 = t2147 * t2148 * t33944;
    let t33953 = -t20539 - 0.174549769648958674e0 * t20542 - t25460 - 0.11557628986739024751e0 * t33941 + 0.58544643236296698111e-1 * t29777 + t20552 + t20561 - 0.58218257753910989057e-2 * t33946 + 0.41917145582815912122e0 * t29781 - 0.16463622957338778997e-1 * t29785 + 0.49390868872016336989e-1 * t29788 - 0.20803732176130244552e1 * t29798 + t25483 + 0.22852785214883496466e0 * t25488;
    (t33953,)
}
