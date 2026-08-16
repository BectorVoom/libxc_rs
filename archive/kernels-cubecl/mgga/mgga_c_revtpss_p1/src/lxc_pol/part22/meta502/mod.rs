//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2238;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2239;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta502<F: Float>(t1089: F, t16183: F, t378: F, t4980: F, t989: F, t4995: F, t1024: F, t1087: F, t1093: F, t11940: F, t12146: F, t15670: F, t15886: F, t16479: F, t16482: F, t16485: F, t16488: F, t16496: F, t16499: F, t16502: F, t16506: F, t16509: F, t3204: F, t3223: F, t3283: F, t3288: F, t3305: F, t3317: F, t381: F, t4743: F, t4967: F, t4977: F, t4984: F, t4999: F, t16237: F, t380: F, t15780: F, t4998: F, t15893: F, t3304: F, t3318: F, t1086: F, t1678: F, t994: F, t12166: F, t342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16515, t16520, t16523, t16526) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2238::<F>(t1089, t16183, t378, t4980, t989, t4995, t1024, t1087, t1093, t11940, t12146, t15670, t15886, t16479, t16482, t16485, t16488, t16496, t16499, t16502, t16506, t16509, t3204, t3223, t3283, t3288, t3305, t3317, t381, t4743, t4967, t4977, t4984, t4999);
        let (t16529, t16534, t16537, t16540, t16543, t16544) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2239::<F>(t16237, t380, t15780, t4998, t15893, t3304, t3318, t1086, t1678, t994);
        let (t16551, t16552) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2240::<F>(t12166, t378, t342);
    (t16515, t16520, t16523, t16526, t16529, t16534, t16537, t16540, t16543, t16544, t16551, t16552)
}
