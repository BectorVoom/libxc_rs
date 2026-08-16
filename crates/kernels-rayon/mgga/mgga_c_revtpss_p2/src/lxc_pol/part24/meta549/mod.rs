//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1622;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1623;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1624;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta549(t50888: f64, t62300: f64, t50892: f64, t50893: f64, t77047: f64, t50901: f64, t40076: f64, t40079: f64, t40184: f64, t40194: f64, t40198: f64, t87673: f64, t1553: f64, t1555: f64, t18592: f64, t18599: f64, t225: f64, t227: f64, t229: f64, t23148: f64, t23227: f64, t23235: f64, t23238: f64, t23241: f64, t2638: f64, t40231: f64, t4415: f64, t4416: f64, t5962: f64, t6006: f64, t6010: f64, t6013: f64, t832: f64, t87543: f64, t87548: f64, t87553: f64, t87634: f64, t87635: f64, t87637: f64, t87645: f64, t87652: f64, t87664: f64, t87672: f64, t231: f64, t10900: f64, t14785: f64, t2721: f64, t2745: f64, t50941: f64, t50943: f64, t5966: f64, t5984: f64, t6035: f64, t62012: f64, t62015: f64, t62029: f64, t62069: f64, t62072: f64, t62089: f64, t62095: f64, t76302: f64, t76767: f64, t76793: f64, t76797: f64, t76804: f64, t76808: f64, t800: f64, t825: f64, t827: f64, t828: f64, t87629: f64, t87394: f64, t40810: f64, t40850: f64, t51042: f64, t51083: f64, t51100: f64, t51104: f64, t62111: f64, t62129: f64, t76812: f64, t76814: f64, t76818: f64, t76823: f64, t76827: f64, t76835: f64, t76856: f64, t76858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87674, t87675, t87676, t87677, t87678, t87679, t87680) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1622(t50888, t62300, t50892, t50893, t77047, t50901, t40076, t40079, t40184, t40194, t40198, t87673);
        let t87713 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1623(t1553, t1555, t18592, t18599, t225, t227, t229, t23148, t23227, t23235, t23238, t23241, t2638, t40231, t4415, t4416, t5962, t6006, t6010, t6013, t832, t87543, t87548, t87553, t87634, t87635, t87637, t87645, t87652, t87664, t87672, t87680);
        let (t87714, t87721) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1624(t231, t87713, t10900, t14785, t2721, t2745, t50941, t50943, t5966, t5984, t6035, t62012, t62015, t62029, t62069, t62072, t62089, t62095, t76302, t76767, t76793, t76797, t76804, t76808, t800, t825, t827, t828, t87629);
        let (t87729, t87742) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1625(t231, t87394, t40810, t40850, t51042, t51083, t51100, t51104, t62111, t62129, t76812, t76814, t76818, t76823, t76827, t76835, t76856, t76858, t825, t827, t828);
    (t87674, t87675, t87676, t87677, t87678, t87679, t87714, t87721, t87729, t87742)
}
