//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2981/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2981<F: Float>(t15707: F, t19920: F, t23891: F, t3127: F, t3172: F, t1043: F, t1063: F, t1066: F, t11250: F, t11632: F, t11927: F, t19620: F, t19677: F, t19930: F, t19934: F, t23976: F, t23992: F, t24007: F, t247: F, t3106: F, t3117: F, t3188: F, t42621: F, t43044: F, t43105: F, t4834: F, t53619: F, t65488: F, t65493: F, t65507: F, t65510: F, t65527: F, t65538: F, t65553: F, t77501: F) -> F {
    let t78910 = t15707 * t19920;
    let t78915 = t3127 * t3172 * t23891;
    let t78954 = -F::cast_from(0.57165357490759649296e-3_f64) * t78910 - F::cast_from(0.42874018118069736972e-3_f64) * t15707 * t19677 - F::cast_from(0.28582678745379824648e-3_f64) * t78915 + F::cast_from(0.25724410870841842183e-2_f64) * t4834 * t19930 - F::cast_from(0.17149607247227894789e-2_f64) * t4834 * t19934 - F::cast_from(0.47637797908966374413e-3_f64) * t65488 + F::cast_from(0.57165357490759649296e-3_f64) * t65493 + F::cast_from(0.17149607247227894788e-2_f64) * t65507 - F::cast_from(0.11433071498151929859e-2_f64) * t65510 - F::cast_from(0.28582678745379824648e-3_f64) * t65527 + F::cast_from(0.14291339372689912324e-3_f64) * t3188 * t23976 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t247 * t1066 * t77501 + F::cast_from(0.57165357490759649295e-3_f64) * t65538 - F::cast_from(0.76220476654346199061e-3_f64) * t3106 * t23976 - F::cast_from(0.57165357490759649295e-3_f64) * t65553 - F::cast_from(0.38586616306262763276e-2_f64) * t42621 * t3117 * t24007 * t11632 * t1043 + F::cast_from(0.38586616306262763276e-2_f64) * t43105 * t3117 * t24007 * t11250 * t1043 - F::cast_from(0.12862205435420921092e-2_f64) * t43044 * t3117 * t24007 * t53619 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t23992 * t19620;
    t78954
}
