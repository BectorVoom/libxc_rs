//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1148/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1148<F: Float>(t6086: F, t7822: F, t31095: F, t31100: F, t31105: F, t35274: F, t35287: F, t39840: F, t39842: F, t39844: F, t39846: F, t39848: F, t39852: F, t39856: F, t39860: F, t39862: F, t39867: F, t39869: F) -> F {
    let t39871 = t7822 * t6086;
    let t39873 = t35274 + F::cast_from(0.34299214494455789578e-2_f64) * t39840 + F::cast_from(0.25724410870841842183e-2_f64) * t39842 - F::cast_from(0.17149607247227894789e-2_f64) * t39844 + F::cast_from(0.25724410870841842183e-2_f64) * t39846 - F::cast_from(0.38586616306262763275e-2_f64) * t39848 - F::cast_from(0.18868855373762491241e-2_f64) * t39852 + F::cast_from(0.37737710747524982482e-2_f64) * t39856 - F::cast_from(0.94344276868812456204e-3_f64) * t39860 + F::cast_from(0.85748036236139473944e-3_f64) * t39862 - F::cast_from(0.85748036236139473944e-3_f64) * t31095 - F::cast_from(0.21437009059034868486e-2_f64) * t31100 + F::cast_from(0.1886885537376249124e-2_f64) * t31105 + F::cast_from(0.13719685797782315831e-1_f64) * t39867 + F::cast_from(0.10289764348336736873e-1_f64) * t39869 - F::cast_from(0.25724410870841842183e-2_f64) * t39871 - t35287;
    t39873
}
