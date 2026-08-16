//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1667;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1668;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta356(t221: f64, t2675: f64, t4343: f64, t2674: f64, t243: f64, t4423: f64, t231: f64, t2662: f64, t2661: f64, t10722: f64, t1565: f64, t4352: f64, t4366: f64, t10726: f64, t10868: f64, t241: f64, t820: f64, t10811: f64, t4452: f64, t2719: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14857, t14859, t14861, t14862, t14864, t14866, t14868) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1667(t221, t2675, t4343, t2674, t243, t4423, t231, t2662, t2661, t10722, t1565, t4352, t4366);
        let (t14869, t14871, t14894) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1668(t10726, t14868, t2661, t10868, t241, t820);
        let (t14907, t14923) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1669(t10811, t4452, t2719, t820, t844);
    (t14857, t14859, t14861, t14862, t14864, t14866, t14869, t14871, t14894, t14907, t14923)
}
