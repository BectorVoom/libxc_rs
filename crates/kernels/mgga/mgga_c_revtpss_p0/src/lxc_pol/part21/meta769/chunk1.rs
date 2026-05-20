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
    let t50106 = F::new(24.0) * t40148;
    let t50113 = t706 * t750 * t13312;
    let t50114 = F::new(12.0) * t50113;
    let t50115 = F::new(3.0) * t40150;
    let t50132 = piecewise3::<F>(t151, F::new(0.0), -F::new(56.0) / F::new(81.0) * t4227 * t10356 + F::new(8.0) / F::new(9.0) * t4230 * t2251 + F::new(8.0) / F::new(9.0) * t1490 * t11231 - F::new(2.0) / F::new(3.0) * t80 * t13312 * t606 - F::new(2.0) / F::new(3.0) * t14447 * t2258 - F::new(2.0) / F::new(9.0) * t4328 * t10326 + F::new(2.0) / F::new(3.0) * t766 * t49889);
    (t50106, t50114, t50115, t50132)
}
