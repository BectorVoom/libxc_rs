//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1753/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1753<F: Float>(t1145: F, t141: F, t89853: F, t12254: F, t89822: F, t68255: F, t68257: F, t81156: F, t81158: F, t89839: F, t89851: F, t89865: F, t89869: F, t89873: F, t89877: F, t90379: F, t90384: F) -> (F, F, F) {
    let t90387 = t141 * t1145 * t89853;
    let t90390 = t141 * t12254 * t89822;
    let t90400 = F::new(0.44152e0) * t90379 + F::cast_from(0.80513333333333333336e0_f64) * t68255 - F::cast_from(0.53675555555555555556e0_f64) * t68257 + F::new(0.298026e1) * t90384 + F::new(0.66228e0) * t90387 + F::new(0.22076e0) * t90390 + F::cast_from(0.80513333333333333333e0_f64) * t81156 - F::new(0.24154e1) * t81158 - F::cast_from(0.60384999999999999999e0_f64) * t89839 + F::new(0.181155e1) * t89851 + F::cast_from(0.40256666666666666666e1_f64) * t89865 - F::new(0.72462e1) * t89869 + F::new(0.72462e1) * t89873 + F::new(0.301925e0) * t89877;
    (t90387, t90390, t90400)
}
