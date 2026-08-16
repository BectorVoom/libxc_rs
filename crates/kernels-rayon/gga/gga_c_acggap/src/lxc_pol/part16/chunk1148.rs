//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1148/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1148(t6086: f64, t7822: f64, t31095: f64, t31100: f64, t31105: f64, t35274: f64, t35287: f64, t39840: f64, t39842: f64, t39844: f64, t39846: f64, t39848: f64, t39852: f64, t39856: f64, t39860: f64, t39862: f64, t39867: f64, t39869: f64) -> f64 {
    let t39871 = t7822 * t6086;
    let t39873 = t35274 + 0.34299214494455789578e-2_f64 * t39840 + 0.25724410870841842183e-2_f64 * t39842 - 0.17149607247227894789e-2_f64 * t39844 + 0.25724410870841842183e-2_f64 * t39846 - 0.38586616306262763275e-2_f64 * t39848 - 0.18868855373762491241e-2_f64 * t39852 + 0.37737710747524982482e-2_f64 * t39856 - 0.94344276868812456204e-3_f64 * t39860 + 0.85748036236139473944e-3_f64 * t39862 - 0.85748036236139473944e-3_f64 * t31095 - 0.21437009059034868486e-2_f64 * t31100 + 0.1886885537376249124e-2_f64 * t31105 + 0.13719685797782315831e-1_f64 * t39867 + 0.10289764348336736873e-1_f64 * t39869 - 0.25724410870841842183e-2_f64 * t39871 - t35287;
    t39873
}
