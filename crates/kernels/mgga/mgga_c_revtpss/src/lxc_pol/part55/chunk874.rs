//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 874/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk874<F: Float>(t1903: F, t7506: F, t7296: F, t27924: F, t27926: F, t27929: F, t25974: F, t25980: F, t25989: F, t25998: F, t26006: F, t26025: F, t26321: F, t26324: F, t26328: F, t27919: F, t27921: F) -> (F, F) {
    let t28862 = t7506 * t1903;
    let t28863 = t7296 * t28862;
    let t28872 = 0.2032800112371413129e-3 * t27924;
    let t28873 = 0.16006300097412701803e-1 * t27926;
    let t28874 = 0.28582678745379824648e-4 * t27929;
    let t28875 = -0.50820002809285328225e-4 * t25998 + t26321 + 0.40015750243531754507e-2 * t26025 + t26328 - t25974 + t25980 + t25989 + 0.17149607247227894789e-1 * t27919 + 0.40015750243531754507e-2 * t27921 + t26006 - t26324 - t28872 + t28873 + t28874;
    (t28863, t28875)
}
