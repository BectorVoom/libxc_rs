//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta739 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2516;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta739<F: Float>(t10760: F, t40627: F, t50613: F, t14861: F, t9794: F, t10890: F, t4458: F, t10815: F, t4426: F, t40424: F, t4430: F, t14720: F, t9775: F, t1561: F, t40360: F, t2682: F, t2719: F, t4368: F, t820: F, t10778: F, t221: F, t2659: F, t4503: F, t816: F, t4372: F, t9784: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51089, t51093, t51096, t51099, t51100, t51102) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2516::<F>(t10760, t40627, t50613, t14861, t9794, t10890, t4458, t10815, t4426, t40424, t4430, t14720, t9775);
        let (t51104, t51122, t51123, t51133, t51170) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2517::<F>(t1561, t40360, t2682, t2719, t4368, t820, t10778, t221, t2659, t4503, t816, t4372, t9784);
    (t51089, t51093, t51096, t51099, t51100, t51102, t51104, t51122, t51123, t51133, t51170)
}
