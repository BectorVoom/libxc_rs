//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1389/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1389<F: Float>(t8590: F, t955: F, t2461: F, t3034: F, t19032: F, t23753: F, t23759: F, t23761: F, t23763: F, t32111: F, t32196: F, t32199: F, t32202: F, t765: F, t19037: F, t19041: F, t19048: F, t19057: F, t19061: F, t22626: F, t26932: F, t32207: F, t32208: F, t32209: F, t32210: F) -> (F, F, F, F) {
    let t33746 = t8590 * t955;
    let t33749 = t3034 * t2461;
    let t33752 = 0.2025780996e0 * t765 * t32111 + t23753 + 0.675260332e-1 * t765 * t32196 + 0.675260332e-1 * t765 * t32199 + 0.2025780996e0 * t765 * t33746 + 0.2025780996e0 * t765 * t33749 + t23759 + t23761 + t23763 - t32202 - t19032;
    let t33756 = t32207 - t19037 - t32208 + t19041 + t19048 + 0.4051561992e0 * t22626 - t32209 + 0.857292e-1 * t26932 + t32210 + t19057 - t19061;
    (t33746, t33749, t33752, t33756)
}
