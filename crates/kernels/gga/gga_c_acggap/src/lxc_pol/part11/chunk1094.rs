//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1094/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1094<F: Float>(t4840: F, t570: F, t30798: F, t30830: F, t30854: F, t30793: F, t30800: F, t30804: F, t30809: F, t30812: F, t30814: F, t30818: F, t30821: F, t30840: F, t30844: F, t30846: F, t30848: F, t30852: F) -> F {
    let t35001 = t570 * t4840;
    let t35004 = F::cast_from(0.21437009059034868486e-3_f64) * t30798;
    let t35012 = F::cast_from(0.20965394859736101379e-2_f64) * t30830;
    let t35018 = F::cast_from(0.25724410870841842184e-2_f64) * t30854;
    let t35019 = -t35001 / F::new(96.0) - F::cast_from(0.37737710747524982481e-2_f64) * t30793 - t35004 + F::cast_from(0.17149607247227894789e-2_f64) * t30800 + F::cast_from(0.18868855373762491241e-2_f64) * t30804 + F::cast_from(0.47172138434406228102e-2_f64) * t30809 + F::cast_from(0.68598428988911579156e-2_f64) * t30812 - F::cast_from(0.68598428988911579156e-2_f64) * t30814 + F::cast_from(0.18868855373762491241e-2_f64) * t30818 + F::cast_from(0.14291339372689912324e-2_f64) * t30821 - t35012 - F::cast_from(0.12862205435420921092e-2_f64) * t30840 - F::cast_from(0.80031500487063509016e-2_f64) * t30844 + F::cast_from(0.31448092289604152068e-2_f64) * t30846 - F::cast_from(0.47172138434406228102e-2_f64) * t30848 - F::cast_from(0.7862023072401038017e-3_f64) * t30852 + t35018;
    t35019
}
