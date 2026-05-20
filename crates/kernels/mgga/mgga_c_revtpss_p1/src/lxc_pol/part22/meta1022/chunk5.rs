//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3566/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3566<F: Float>(t20112: F, t342: F, t15669: F, t1678: F, t1076: F, t1079: F, t1097: F, t11121: F, t16152: F, t16255: F, t16275: F, t16333: F, t16597: F, t1696: F, t19429: F, t20219: F, t3058: F, t3060: F, t3063: F, t3270: F, t3325: F, t33754: F, t4752: F, t4941: F, t4947: F, t53015: F, t53034: F, t53174: F, t53180: F, t6244: F, t6392: F) -> F {
    let t68138 = t342 * t20112;
    let t68144 = t15669 * t1678;
    let t68163 = F::cast_from(0.52683593463484092788e1_f64) * t4752 * t16255 + F::cast_from(0.52683593463484092788e1_f64) * t16333 * t4947 - F::cast_from(0.79025390195226139182e1_f64) * t53015 * t16275 - F::cast_from(0.13170898365871023197e1_f64) * t68138 * t1097 - F::cast_from(0.15805078039045227836e2_f64) * t53174 * t33754 * t16152 + F::cast_from(0.26341796731742046394e1_f64) * t68144 * t3060 - F::cast_from(0.13170898365871023197e1_f64) * t3058 * t1079 * t6244 * t3325 - F::cast_from(0.52683593463484092788e1_f64) * t53034 * t19429 - F::cast_from(0.13170898365871023197e1_f64) * t53180 * t1696 - F::cast_from(0.39512695097613069591e1_f64) * t1076 * t11121 * t6392 * t3270 + F::cast_from(0.13170898365871023197e1_f64) * t3063 * t20219 + F::cast_from(0.26341796731742046394e1_f64) * t16597 * t4941;
    t68163
}
