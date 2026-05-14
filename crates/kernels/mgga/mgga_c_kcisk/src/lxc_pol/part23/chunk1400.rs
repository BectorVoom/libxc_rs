//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1400/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1400<F: Float>(t114849: F, t1520: F, t20934: F, t32229: F, t4534: F, t9878: F, t2732: F, t48697: F, t114829: F, t114831: F, t114837: F, t114838: F, t114840: F, t114841: F, t114844: F, t15094: F, t21345: F, t22153: F, t32523: F, t32529: F, t32533: F, t33757: F, t41861: F, t41864: F, t4536: F, t4565: F, t57167: F, t6638: F, t9560: F, t9882: F) -> (F, F, F, F) {
    let t114851 = 2.0 * t114849 * t1520;
    let t114853 = 4.0 * t32229 * t20934;
    let t114859 = t9878 * t4534;
    let t114862 = t48697 * t2732;
    let t114863 = -6.0 * t15094 * t4565 * t9882 + 2.0 * t114859 * t4536 + 2.0 * t21345 * t32529 + 4.0 * t22153 * t32523 - 2.0 * t32533 * t6638 - 12.0 * t33757 * t41861 + 2.0 * t41864 * t9882 + 4.0 * t57167 * t9560 + t114829 - t114831 + t114837 + t114838 - t114840 + t114841 + t114844 + t114851 - t114853 + t114862;
    (t114851, t114853, t114862, t114863)
}
