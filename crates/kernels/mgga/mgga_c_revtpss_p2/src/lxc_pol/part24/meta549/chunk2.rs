//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1624/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1624<F: Float>(t231: F, t87713: F, t10900: F, t14785: F, t2721: F, t2745: F, t50941: F, t50943: F, t5966: F, t5984: F, t6035: F, t62012: F, t62015: F, t62029: F, t62069: F, t62072: F, t62089: F, t62095: F, t76302: F, t76767: F, t76793: F, t76797: F, t76804: F, t76808: F, t800: F, t825: F, t827: F, t828: F, t87629: F) -> (F, F) {
    let t87714 = t87713 * t231;
    let t87721 = F::cast_from(0.40015750243531754508e-2_f64) * t76767 - F::cast_from(0.51448821741683684368e-1_f64) * t2745 * t14785 * t76302 * t6035 - F::new(3.0) / F::new(2.0) * t10900 * t800 * t5984 * t5966 + F::new(455.0) / F::new(162.0) * t50941 + F::cast_from(0.54214778996945588149e-4_f64) * t62012 - F::cast_from(0.27107389498472794074e-4_f64) * t62015 - F::cast_from(0.73180804045370872643e-3_f64) * t50943 - F::cast_from(0.65049603595885220128e-2_f64) * t62029 + F::cast_from(0.15246000842785598467e-4_f64) * t62069 - F::cast_from(0.30492001685571196935e-4_f64) * t62072 + F::new(35.0) / F::new(12.0) * t62089 - F::new(35.0) / F::new(36.0) * t62095 + F::cast_from(0.40015750243531754508e-2_f64) * t76793 - F::cast_from(0.34299214494455789577e-3_f64) * t76797 + F::cast_from(0.30011812682648815881e-2_f64) * t2721 * t827 * t828 * t87629 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t87714 + F::cast_from(0.48018900292238105409e0_f64) * t76804 - F::cast_from(0.6098400337114239387e-2_f64) * t76808;
    (t87714, t87721)
}
