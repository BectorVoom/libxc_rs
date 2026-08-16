//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3011/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3011(t80008: f64, t80027: f64, t1011: f64, t1025: f64, t20039: f64, t23869: f64, t23874: f64, t24024: f64, t24034: f64, t3224: f64, t3241: f64, t371: f64, t372: f64, t373: f64, t42262: f64, t43069: f64, t4915: f64, t4919: f64, t66689: f64, t67327: f64, t67329: f64, t67353: f64, t67355: f64, t67358: f64, t67426: f64, t77525: f64, t77529: f64, t77533: f64, t77537: f64, t77588: f64, t77592: f64, t79957: f64) -> (f64, f64) {
    let t80028 = t80008 + t80027;
    let t80034 = -0.85748036236139473944e-3_f64 * t67327 - 0.85748036236139473944e-3_f64 * t67329 - t3241 * t23869 / 108.0_f64 + t79957 / 864.0_f64 - 7.0_f64 / 243.0_f64 * t3241 * t23874 + 0.85748036236139473944e-3_f64 * t67353 + 0.17149607247227894789e-2_f64 * t67355 - 0.95275595817932748825e-3_f64 * t67358 + t1011 * t4919 * t77529 / 72.0_f64 + t1011 * t4919 * t77525 / 72.0_f64 - t1011 * t4919 * t77592 / 12.0_f64 - t1011 * t4915 * t77533 / 12.0_f64 + t1011 * t4915 * t77537 / 16.0_f64 + t1011 * t4919 * t77588 / 6.0_f64 - 0.42874018118069736972e-3_f64 * t67426 - 0.12862205435420921092e-2_f64 * t42262 * t24034 + 0.17149607247227894789e-2_f64 * t43069 * t66689 * t20039 - 0.21437009059034868486e-3_f64 * t3224 * t24024 - 0.21437009059034868486e-3_f64 * t1025 * t371 * t372 * t373 * t80028;
    (t80028, t80034)
}
