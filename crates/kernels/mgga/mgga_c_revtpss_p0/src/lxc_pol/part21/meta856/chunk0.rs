//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3247/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3247<F: Float>(t10355: F, t44: F, t10368: F, t56: F, t10326: F, t10345: F, t10369: F, t10376: F, t13312: F, t13313: F, t13324: F, t1474: F, t1480: F, t2258: F, t2270: F, t2282: F, t4205: F, t4210: F, t46090: F, t48: F, t49889: F, t51959: F, t60: F, t606: F, t614: F) -> F {
    let t60308 = t44 * t10355;
    let t60311 = t56 * t10368;
    let t60330 = F::new(5.0) / F::new(6.0) * t56 * t2282 * t13312 * t606 + F::new(5.0) / F::new(6.0) * t56 * t13324 * t2258 + F::new(5.0) / F::new(18.0) * t56 * t4210 * t10326 - t46090 - F::new(5.0) / F::new(36.0) * t60308 * t51959 + F::new(5.0) / F::new(36.0) * t60311 * t51959 + F::new(220.0) / F::new(9.0) * t2270 * t4205 - F::new(20.0) / F::new(3.0) * t614 * t13313 + F::new(5.0) / F::new(6.0) * t44 * t48 * t49889 - F::new(10.0) / F::new(81.0) * t1480 * t10369 + F::new(20.0) / F::new(9.0) * t1480 * t10376 - F::new(5.0) / F::new(6.0) * t56 * t60 * t49889 - F::new(3080.0) / F::new(81.0) * t10345 * t1474;
    t60330
}
