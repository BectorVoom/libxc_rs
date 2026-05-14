//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1056/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1056<F: Float>(t15086: F, t5180: F, t15061: F, t15063: F, t15066: F, t15069: F, t15072: F, t15074: F, t15076: F, t15078: F, t15080: F, t15083: F, t14738: F, t14740: F, t14743: F, t14745: F, t14747: F, t14749: F, t14751: F, t14754: F, t14756: F, t14760: F, t14762: F, t14796: F, t14856: F, t14858: F, t14861: F, t14863: F, t14866: F, t14869: F, t14872: F, t14876: F, t14879: F, t14882: F, t15058: F) -> (F, F) {
    let t15087 = t5180 * t15086;
    let t15089 = -t15061 / 192.0 + t15063 / 18.0 - t15066 / 64.0 - t15069 / 12.0 - 11.0 / 18.0 * t15072 - t15074 / 24.0 - t15076 / 24.0 + t15078 / 256.0 + 2.0 / 9.0 * t15080 + t15083 / 8.0 + t15087 / 864.0;
    let t15092 = t14738 + t14740 / 96.0 + t14743 / 36.0 - t14745 / 16.0 + t14747 / 96.0 + t14749 / 24.0 - t14751 / 8.0 + 11.0 / 27.0 * t14754 + t14756 / 128.0 - t14760 / 256.0 + t14762 / 12.0 + t14796 + t14856 + 2.0 / 27.0 * t14858 - 2.0 / 3.0 * t14861 + t14863 / 24.0 - t14866 / 24.0 + t14869 / 54.0 - t14872 / 288.0 - t14876 / 8.0 + t14879 / 4.0 + t14882 / 3.0 + t15058 / 16.0 + t15089;
    (t15087, t15092)
}
