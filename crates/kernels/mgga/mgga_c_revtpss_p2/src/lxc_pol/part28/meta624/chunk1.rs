//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2215/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2215<F: Float>(t100007: F, t16094: F, t12167: F, t99984: F, t12078: F, t25516: F, t4954: F, t15752: F, t27498: F, t15596: F, t15601: F, t15615: F, t15910: F, t15965: F, t16084: F, t16128: F, t16201: F, t25517: F, t3097: F, t4788: F, t4907: F, t7132: F, t93670: F, t93821: F) -> (F, F) {
    let t100135 = t16094 * t100007;
    let t100138 = t12167 * t99984;
    let t100141 = t12078 * t99984;
    let t100146 = t4954 * t25516;
    let t100160 = F::cast_from(0.57165357490759649296e-3_f64) * t27498 * t15752;
    let t100163 = -F::cast_from(0.95275595817932748826e-3_f64) * t100135 * t16128 + F::cast_from(0.25724410870841842183e-2_f64) * t100138 * t16084 - F::cast_from(0.25724410870841842183e-2_f64) * t100141 * t15910 - F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t15965 + F::cast_from(0.57165357490759649296e-3_f64) * t100146 * t3097 + F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t15615 + F::cast_from(0.57165357490759649296e-3_f64) * t93821 * t4788 + F::cast_from(0.47637797908966374413e-3_f64) * t25517 * t15596 + F::cast_from(0.28582678745379824648e-3_f64) * t25517 * t15601 - F::cast_from(0.28582678745379824648e-2_f64) * t7132 * t16201 - t100160 + F::cast_from(0.45732285992607719436e-2_f64) * t93670 * t4907;
    (t100135, t100163)
}
