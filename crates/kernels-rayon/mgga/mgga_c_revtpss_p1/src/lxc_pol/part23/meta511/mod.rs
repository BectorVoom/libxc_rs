//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2006;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2007;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2008;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2009;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta511(t21013: f64, t3767: f64, t3782: f64, t3628: f64, t4186: f64, t5351: f64, t3626: f64, t12910: f64, t17283: f64, t17375: f64, t17448: f64, t17605: f64, t1791: f64, t21001: f64, t21004: f64, t21008: f64, t3625: f64, t5320: f64, t5323: f64, t5335: f64, t5343: f64, t5402: f64, t5407: f64, t12712: f64, t471: f64, t6688: f64, t3720: f64, t1774: f64, t3367: f64, t4181: f64, t6622: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t21014 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2006(t21013, t3767);
        let t21017 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2007(t21013, t3782);
        let (t21020, t21021, t21022, t21027) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2008(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
        let t21028 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2009(t12712, t471);
        let (t21029, t21030, t21036, t21037, t21040) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2010(t21028, t6688, t3720, t1774, t3367, t4181, t3626, t6622, t73);
    (t21014, t21017, t21020, t21021, t21022, t21027, t21028, t21029, t21030, t21036, t21037, t21040)
}
