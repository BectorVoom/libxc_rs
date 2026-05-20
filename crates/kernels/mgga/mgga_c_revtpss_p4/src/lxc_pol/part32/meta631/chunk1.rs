//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2044/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2044<F: Float>(t5883: F, t7356: F, t108710: F, t109153: F, t109242: F, t13426: F, t18227: F, t2055: F, t2322: F, t27123: F, t28219: F, t28683: F, t30138: F, t30143: F, t30570: F, t4248: F, t5523: F, t7373: F, t7889: F, t7983: F) -> (F, F) {
    let t111066 = t7356 * t5883;
    let t111068 = F::new(2.0) * t108710 * t2055 + F::new(4.0) * t109153 * t2055 + F::new(2.0) * t109242 * t2055 + F::new(4.0) * t13426 * t7983 + F::new(4.0) * t18227 * t7983 + F::new(2.0) * t2322 * t30570 + F::new(4.0) * t27123 * t7983 + F::new(4.0) * t28219 * t7983 + F::new(4.0) * t28683 * t4248 + F::new(4.0) * t28683 * t7889 + F::new(4.0) * t30138 * t7373 + F::new(2.0) * t30143 * t7373 + F::new(2.0) * t30570 * t5523 + F::new(2.0) * t111066;
    (t111066, t111068)
}
