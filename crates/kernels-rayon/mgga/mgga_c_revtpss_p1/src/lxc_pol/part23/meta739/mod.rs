//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta739 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2516;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta739(t10760: f64, t40627: f64, t50613: f64, t14861: f64, t9794: f64, t10890: f64, t4458: f64, t10815: f64, t4426: f64, t40424: f64, t4430: f64, t14720: f64, t9775: f64, t1561: f64, t40360: f64, t2682: f64, t2719: f64, t4368: f64, t820: f64, t10778: f64, t221: f64, t2659: f64, t4503: f64, t816: f64, t4372: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51089, t51093, t51096, t51099, t51100, t51102) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2516(t10760, t40627, t50613, t14861, t9794, t10890, t4458, t10815, t4426, t40424, t4430, t14720, t9775);
        let (t51104, t51122, t51123, t51133, t51170) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2517(t1561, t40360, t2682, t2719, t4368, t820, t10778, t221, t2659, t4503, t816, t4372, t9784);
    (t51089, t51093, t51096, t51099, t51100, t51102, t51104, t51122, t51123, t51133, t51170)
}
