//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1406/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1406<F: Float>(t34782: F, t34785: F, t34788: F, t34791: F, t34794: F, t34804: F, t34772: F, t34776: F, t34797: F, t34802: F, t37072: F, t34808: F) -> (F, F) {
    let t37073 = F::cast_from(0.20041830772435757309e-6_f64) * t34782;
    let t37074 = F::cast_from(0.69504740211613770836e-3_f64) * t34785;
    let t37075 = F::cast_from(0.50083268227528753081e-5_f64) * t34788;
    let t37076 = F::cast_from(0.43440462632258606772e-4_f64) * t34791;
    let t37077 = F::cast_from(0.11372686522837130914e-4_f64) * t34794;
    let t37080 = F::cast_from(0.9275345110817126956e-4_f64) * t34804;
    let t37081 = F::cast_from(0.19336854506021130163e-7_f64) * t34772 - F::cast_from(0.52389984474979915324e-9_f64) * t34776 - t37072 - t37073 - t37074 + t37075 + t37076 + t37077 + F::cast_from(0.29465683056794103106e-8_f64) * t34797 - F::cast_from(0.98332751566569010434e-8_f64) * t34802 + t37080;
    let t37082 = F::cast_from(0.77294542590142724634e-6_f64) * t34808;
    (t37081, t37082)
}
