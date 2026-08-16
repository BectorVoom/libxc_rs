//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1094/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1094(t4840: f64, t570: f64, t30798: f64, t30830: f64, t30854: f64, t30793: f64, t30800: f64, t30804: f64, t30809: f64, t30812: f64, t30814: f64, t30818: f64, t30821: f64, t30840: f64, t30844: f64, t30846: f64, t30848: f64, t30852: f64) -> f64 {
    let t35001 = t570 * t4840;
    let t35004 = 0.21437009059034868486e-3_f64 * t30798;
    let t35012 = 0.20965394859736101379e-2_f64 * t30830;
    let t35018 = 0.25724410870841842184e-2_f64 * t30854;
    let t35019 = -t35001 / 96.0_f64 - 0.37737710747524982481e-2_f64 * t30793 - t35004 + 0.17149607247227894789e-2_f64 * t30800 + 0.18868855373762491241e-2_f64 * t30804 + 0.47172138434406228102e-2_f64 * t30809 + 0.68598428988911579156e-2_f64 * t30812 - 0.68598428988911579156e-2_f64 * t30814 + 0.18868855373762491241e-2_f64 * t30818 + 0.14291339372689912324e-2_f64 * t30821 - t35012 - 0.12862205435420921092e-2_f64 * t30840 - 0.80031500487063509016e-2_f64 * t30844 + 0.31448092289604152068e-2_f64 * t30846 - 0.47172138434406228102e-2_f64 * t30848 - 0.7862023072401038017e-3_f64 * t30852 + t35018;
    t35019
}
