//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1622;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1623;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1624;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta549<F: Float>(t50888: F, t62300: F, t50892: F, t50893: F, t77047: F, t50901: F, t40076: F, t40079: F, t40184: F, t40194: F, t40198: F, t87673: F, t1553: F, t1555: F, t18592: F, t18599: F, t225: F, t227: F, t229: F, t23148: F, t23227: F, t23235: F, t23238: F, t23241: F, t2638: F, t40231: F, t4415: F, t4416: F, t5962: F, t6006: F, t6010: F, t6013: F, t832: F, t87543: F, t87548: F, t87553: F, t87634: F, t87635: F, t87637: F, t87645: F, t87652: F, t87664: F, t87672: F, t231: F, t10900: F, t14785: F, t2721: F, t2745: F, t50941: F, t50943: F, t5966: F, t5984: F, t6035: F, t62012: F, t62015: F, t62029: F, t62069: F, t62072: F, t62089: F, t62095: F, t76302: F, t76767: F, t76793: F, t76797: F, t76804: F, t76808: F, t800: F, t825: F, t827: F, t828: F, t87629: F, t87394: F, t40810: F, t40850: F, t51042: F, t51083: F, t51100: F, t51104: F, t62111: F, t62129: F, t76812: F, t76814: F, t76818: F, t76823: F, t76827: F, t76835: F, t76856: F, t76858: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87674, t87675, t87676, t87677, t87678, t87679, t87680) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1622::<F>(t50888, t62300, t50892, t50893, t77047, t50901, t40076, t40079, t40184, t40194, t40198, t87673);
        let t87713 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1623::<F>(t1553, t1555, t18592, t18599, t225, t227, t229, t23148, t23227, t23235, t23238, t23241, t2638, t40231, t4415, t4416, t5962, t6006, t6010, t6013, t832, t87543, t87548, t87553, t87634, t87635, t87637, t87645, t87652, t87664, t87672, t87680);
        let (t87714, t87721) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1624::<F>(t231, t87713, t10900, t14785, t2721, t2745, t50941, t50943, t5966, t5984, t6035, t62012, t62015, t62029, t62069, t62072, t62089, t62095, t76302, t76767, t76793, t76797, t76804, t76808, t800, t825, t827, t828, t87629);
        let (t87729, t87742) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1625::<F>(t231, t87394, t40810, t40850, t51042, t51083, t51100, t51104, t62111, t62129, t76812, t76814, t76818, t76823, t76827, t76835, t76856, t76858, t825, t827, t828);
    (t87674, t87675, t87676, t87677, t87678, t87679, t87714, t87721, t87729, t87742)
}
