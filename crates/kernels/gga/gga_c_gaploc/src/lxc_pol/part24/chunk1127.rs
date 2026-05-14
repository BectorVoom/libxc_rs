//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1127/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1127<F: Float>(t22144: F, t32145: F, t549: F, t7390: F, t8756: F, t6111: F, t8769: F, t2365: F, t24745: F, t10867: F, t29021: F, t29030: F, t3040: F, t10627: F, t1880: F, t23335: F, t6066: F) -> (F, F, F, F, F, F, F, F) {
    let t32875 = 0.2044956050875773316e1 * t22144 * t32145;
    let t32877 = t7390 * t549 * t8756;
    let t32878 = 0.59584149919750711116e-1 * t32877;
    let t32880 = t6111 * t549 * t8769;
    let t32881 = 0.11916829983950142223e0 * t32880;
    let t32883 = t6111 * t2365 * t24745;
    let t32884 = 0.29792074959875355558e-1 * t32883;
    let t32885 = t10867 * t29021;
    let t32886 = 0.10427226235956374445e0 * t32885;
    let t32888 = 0.35750489951850426669e0 * t29030 * t3040;
    let t32889 = t10627 * t1880;
    let t32892 = 0.14300195980740170668e1 * t23335 * t6066 * t32889;
    (t32875, t32878, t32881, t32884, t32886, t32888, t32889, t32892)
}
