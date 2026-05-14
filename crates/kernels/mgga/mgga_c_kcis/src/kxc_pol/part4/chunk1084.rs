//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1084/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1084<F: Float>(t15061: F, t15063: F, t15066: F, t15069: F, t15072: F, t15074: F, t15076: F, t15078: F, t15080: F, t15083: F, t15087: F, t14740: F, t14743: F, t14745: F, t14747: F, t14749: F, t14751: F, t14754: F, t14756: F, t14760: F, t14762: F, t14858: F, t14861: F, t14863: F, t14866: F, t14869: F, t14872: F, t14876: F, t14879: F, t14882: F, t15058: F, t15716: F, t15739: F, t15762: F) -> (F,) {
    let t15785 = -0.13489583333333333333e-1 * t15061 + 0.14388888888888888889e0 * t15063 - 0.4046875e-1 * t15066 - 0.125e0 * t15069 - 0.91666666666666666667e0 * t15072 - 0.10791666666666666667e0 * t15074 - 0.10791666666666666667e0 * t15076 + 0.101171875e-1 * t15078 + 0.33333333333333333334e0 * t15080 + 0.1875e0 * t15083 + 0.29976851851851851851e-2 * t15087;
    let t15788 = t15716 + 0.26979166666666666666e-1 * t14740 + 0.41666666666666666666e-1 * t14743 - 0.9375e-1 * t14745 + 0.26979166666666666666e-1 * t14747 + 0.625e-1 * t14749 - 0.1875e0 * t14751 + 0.61111111111111111112e0 * t14754 + 0.20234375e-1 * t14756 - 0.101171875e-1 * t14760 + 0.125e0 * t14762 + t15739 + t15762 + 0.11111111111111111111e0 * t14858 - 1.0 * t14861 + 0.10791666666666666667e0 * t14863 - 0.10791666666666666667e0 * t14866 + 0.47962962962962962962e-1 * t14869 - 0.89930555555555555554e-2 * t14872 - 0.1875e0 * t14876 + 0.375e0 * t14879 + 0.5e0 * t14882 + 0.9375e-1 * t15058 + t15785;
    (t15788,)
}
