//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 762/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk762<F: Float>(t8847: F, t8851: F, t8860: F, t8233: F, t8835: F, t8839: F, t8843: F, t8845: F, t8849: F, t8856: F, t8862: F, t8864: F, t8866: F, t8870: F, t9313: F, t8876: F) -> (F, F) {
    let t9316 = 0.17149607247227894789e-2 * t8847;
    let t9318 = 0.21437009059034868486e-3 * t8851;
    let t9320 = 0.14291339372689912324e-3 * t8860;
    let t9325 = -t8233 + 0.40015750243531754507e-2 * t8835 - 0.10718504529517434243e-2 * t8839 + t9313 + 0.17149607247227894789e-2 * t8843 + 0.17149607247227894789e-2 * t8845 - t9316 - 0.17149607247227894789e-2 * t8849 + t9318 + 0.21437009059034868486e-3 * t8856 + t9320 + 0.37737710747524982483e-2 * t8862 - t8864 / 48.0 - t8866 / 24.0 + 0.31448092289604152069e-3 * t8870;
    let t9328 = t8876 / 32.0;
    (t9325, t9328)
}
