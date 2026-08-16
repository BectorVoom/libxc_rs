//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2165;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta525(t4995: f64, t989: f64, t1024: f64, t1087: f64, t1093: f64, t11940: f64, t12146: f64, t15670: f64, t15886: f64, t16479: f64, t16482: f64, t16485: f64, t16488: f64, t16496: f64, t16499: f64, t16502: f64, t16506: f64, t16509: f64, t16515: f64, t16520: f64, t3204: f64, t3223: f64, t3283: f64, t3288: f64, t3305: f64, t3317: f64, t381: f64, t4743: f64, t4967: f64, t4977: f64, t4984: f64, t4999: f64, t16237: f64, t380: f64, t15780: f64, t4998: f64, t15893: f64, t3304: f64, t3318: f64, t1086: f64, t1678: f64, t994: f64, t12166: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16523, t16526) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2165(t4995, t989, t1024, t1087, t1093, t11940, t12146, t15670, t15886, t16479, t16482, t16485, t16488, t16496, t16499, t16502, t16506, t16509, t16515, t16520, t3204, t3223, t3283, t3288, t3305, t3317, t381, t4743, t4967, t4977, t4984, t4999);
        let (t16529, t16534, t16537, t16540, t16543, t16544, t16551) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2166(t16237, t380, t15780, t4998, t15893, t3304, t3318, t1086, t1678, t994, t12166, t378);
    (t16523, t16526, t16529, t16534, t16537, t16540, t16543, t16544, t16551)
}
