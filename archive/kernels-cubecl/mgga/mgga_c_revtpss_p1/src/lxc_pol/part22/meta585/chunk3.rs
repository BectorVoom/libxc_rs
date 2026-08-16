//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2454/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2454<F: Float>(t18687: F, t18722: F, t18754: F, t18782: F, t868: F, t10503: F, t10507: F, t10511: F, t10984: F, t14998: F, t15004: F, t15006: F, t15010: F, t15015: F, t18324: F, t18658: F, t18663: F, t213: F, t257: F, t865: F) -> (F, F, F) {
    let t18784 = t18687 + t18722 + t18754 + t18782;
    let t18785 = t868 * t18784;
    let t18791 = F::cast_from(0.13170898365871023197e1_f64) * t865 * t18324 - F::cast_from(0.14634331517634470219e-1_f64) * t14998 - t10503 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t18658 * t257 - F::cast_from(0.39512695097613069591e1_f64) * t865 * t18663 - F::cast_from(0.11565819519348392139e-2_f64) * t10507 + F::cast_from(0.13009920719177044025e-1_f64) * t10511 - F::cast_from(0.65854491829355115987e0_f64) * t865 * t18785 - F::cast_from(0.23131639038696784278e-2_f64) * t15004 + t10984 - F::cast_from(0.26019841438354088051e-1_f64) * t15006 + t15010 + F::cast_from(0.13009920719177044025e-2_f64) * t15015;
    (t18784, t18785, t18791)
}
