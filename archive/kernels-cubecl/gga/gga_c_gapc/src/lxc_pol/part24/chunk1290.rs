//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1290/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1290<F: Float>(t35890: F, t35895: F, t35898: F, t35901: F, t35903: F, t35907: F, t35909: F, t35912: F, t35915: F, t35919: F, t35921: F, t35923: F, t35925: F) -> F {
    let t37570 = -F::cast_from(0.13678971311312682431e-5_f64) * t35890 - F::cast_from(0.12311074180181414188e-4_f64) * t35895 - F::cast_from(0.19571635326701179828e-6_f64) * t35898 + F::cast_from(0.25687771366295298524e-5_f64) * t35901 - F::cast_from(0.93943849568165663175e-3_f64) * t35903 - F::cast_from(0.25687771366295298524e-5_f64) * t35907 + F::cast_from(0.93943849568165663176e-4_f64) * t35909 - F::cast_from(0.25687771366295298524e-5_f64) * t35912 + F::cast_from(0.23914252259458958957e-6_f64) * t35915 + F::cast_from(0.29357452990051769742e-5_f64) * t35919 - F::cast_from(0.64586396578113893434e-4_f64) * t35921 - F::cast_from(0.3757753982726626527e-3_f64) * t35923 - F::cast_from(0.40598095546020480691e-5_f64) * t35925;
    t37570
}
