//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1885;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta454(t4982: f64, t999: f64, t19501: f64, t1024: f64, t1083: f64, t1087: f64, t11940: f64, t12122: f64, t12149: f64, t16544: f64, t16559: f64, t16566: f64, t19438: f64, t19443: f64, t19447: f64, t19453: f64, t19457: f64, t19463: f64, t19479: f64, t19484: f64, t19488: f64, t19492: f64, t19498: f64, t3223: f64, t3287: f64, t4857: f64, t4954: f64, t4977: f64, t4988: f64, t4992: f64, t4996: f64, t5005: f64, t6368: f64, t4757: f64, t5004: f64, t3291: f64, t6244: f64, t1082: f64, t19399: f64, t4866: f64, t4893: f64, t1647: f64, t4980: f64, t1071: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19502, t19503, t19508) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1885(t4982, t999, t19501, t1024, t1083, t1087, t11940, t12122, t12149, t16544, t16559, t16566, t19438, t19443, t19447, t19453, t19457, t19463, t19479, t19484, t19488, t19492, t19498, t3223, t3287, t4857, t4954, t4977, t4988, t4992, t4996, t5005, t6368);
        let (t19509, t19512, t19515, t19520, t19521, t19526, t19533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1886(t4757, t5004, t3291, t6244, t1082, t19399, t4866, t4982, t4893, t1647, t4980, t1071, t6305);
    (t19502, t19503, t19508, t19509, t19512, t19515, t19520, t19521, t19526, t19533)
}
