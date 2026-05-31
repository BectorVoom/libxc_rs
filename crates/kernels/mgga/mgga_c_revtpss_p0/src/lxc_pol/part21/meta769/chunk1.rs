//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2724/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2724<F: Float>(t45: F, t40148: F, t13312: F, t706: F, t750: F, t40150: F, t10326: F, t10356: F, t11231: F, t14447: F, t1490: F, t2251: F, t2258: F, t4227: F, t4230: F, t4328: F, t49889: F, t606: F, t766: F, t80: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t50106 = F::cast_from(24.0_f64) * t40148;
    let t50113 = t706 * t750 * t13312;
    let t50114 = F::cast_from(12.0_f64) * t50113;
    let t50115 = F::cast_from(3.0_f64) * t40150;
    let t50132 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t4227 * t10356 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4230 * t2251 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1490 * t11231 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t80 * t13312 * t606 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14447 * t2258 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4328 * t10326 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t49889);
    (t50106, t50114, t50115, t50132)
}
