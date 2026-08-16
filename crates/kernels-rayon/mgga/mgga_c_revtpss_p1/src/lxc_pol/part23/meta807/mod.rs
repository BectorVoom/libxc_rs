//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta807 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2640;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta807(t2801: f64, t62967: f64, t14563: f64, t14568: f64, t14598: f64, t14600: f64, t4423: f64, t676: f64, t14602: f64, t2482: f64, t2811: f64, t6016: f64, t10535: f64, t136: f64, t2457: f64, t6017: f64, t10542: f64, t18726: f64, t2439: f64, t2440: f64, t6072: f64, t2444: f64, t689: f64, t15003: f64, t51258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62968, t62983, t62987, t62992) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2640(t2801, t62967, t14563, t14568, t14598, t14600, t4423, t676, t14602, t2482, t2811, t6016);
        let (t62999, t63015, t63050, t63053, t63058) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2641(t10535, t136, t2457, t6017, t10542, t18726, t2439, t2440, t6072, t2444, t689, t15003, t51258);
    (t62968, t62983, t62987, t62992, t62999, t63015, t63050, t63053, t63058)
}
