//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3011/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3011<F: Float>(t80008: F, t80027: F, t1011: F, t1025: F, t20039: F, t23869: F, t23874: F, t24024: F, t24034: F, t3224: F, t3241: F, t371: F, t372: F, t373: F, t42262: F, t43069: F, t4915: F, t4919: F, t66689: F, t67327: F, t67329: F, t67353: F, t67355: F, t67358: F, t67426: F, t77525: F, t77529: F, t77533: F, t77537: F, t77588: F, t77592: F, t79957: F) -> (F, F) {
    let t80028 = t80008 + t80027;
    let t80034 = -F::cast_from(0.85748036236139473944e-3_f64) * t67327 - F::cast_from(0.85748036236139473944e-3_f64) * t67329 - t3241 * t23869 / F::new(108.0) + t79957 / F::new(864.0) - F::new(7.0) / F::new(243.0) * t3241 * t23874 + F::cast_from(0.85748036236139473944e-3_f64) * t67353 + F::cast_from(0.17149607247227894789e-2_f64) * t67355 - F::cast_from(0.95275595817932748825e-3_f64) * t67358 + t1011 * t4919 * t77529 / F::new(72.0) + t1011 * t4919 * t77525 / F::new(72.0) - t1011 * t4919 * t77592 / F::new(12.0) - t1011 * t4915 * t77533 / F::new(12.0) + t1011 * t4915 * t77537 / F::new(16.0) + t1011 * t4919 * t77588 / F::new(6.0) - F::cast_from(0.42874018118069736972e-3_f64) * t67426 - F::cast_from(0.12862205435420921092e-2_f64) * t42262 * t24034 + F::cast_from(0.17149607247227894789e-2_f64) * t43069 * t66689 * t20039 - F::cast_from(0.21437009059034868486e-3_f64) * t3224 * t24024 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t371 * t372 * t373 * t80028;
    (t80028, t80034)
}
