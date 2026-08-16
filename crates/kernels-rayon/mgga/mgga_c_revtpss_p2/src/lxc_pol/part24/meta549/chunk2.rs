//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1624/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1624(t231: f64, t87713: f64, t10900: f64, t14785: f64, t2721: f64, t2745: f64, t50941: f64, t50943: f64, t5966: f64, t5984: f64, t6035: f64, t62012: f64, t62015: f64, t62029: f64, t62069: f64, t62072: f64, t62089: f64, t62095: f64, t76302: f64, t76767: f64, t76793: f64, t76797: f64, t76804: f64, t76808: f64, t800: f64, t825: f64, t827: f64, t828: f64, t87629: f64) -> (f64, f64) {
    let t87714 = t87713 * t231;
    let t87721 = 0.40015750243531754508e-2_f64 * t76767 - 0.51448821741683684368e-1_f64 * t2745 * t14785 * t76302 * t6035 - 3.0_f64 / 2.0_f64 * t10900 * t800 * t5984 * t5966 + 455.0_f64 / 162.0_f64 * t50941 + 0.54214778996945588149e-4_f64 * t62012 - 0.27107389498472794074e-4_f64 * t62015 - 0.73180804045370872643e-3_f64 * t50943 - 0.65049603595885220128e-2_f64 * t62029 + 0.15246000842785598467e-4_f64 * t62069 - 0.30492001685571196935e-4_f64 * t62072 + 35.0_f64 / 12.0_f64 * t62089 - 35.0_f64 / 36.0_f64 * t62095 + 0.40015750243531754508e-2_f64 * t76793 - 0.34299214494455789577e-3_f64 * t76797 + 0.30011812682648815881e-2_f64 * t2721 * t827 * t828 * t87629 - 0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t87714 + 0.48018900292238105409e0_f64 * t76804 - 0.6098400337114239387e-2_f64 * t76808;
    (t87714, t87721)
}
