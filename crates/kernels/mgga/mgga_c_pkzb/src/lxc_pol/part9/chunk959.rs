//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 959/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk959<F: Float>(t261: F, t7510: F, t7335: F, t5522: F, t5525: F, t5812: F, t7352: F, t7357: F, t1918: F, t1957: F, t1972: F, t248: F, t2829: F, t704: F, t714: F, t723: F, t7446: F, t7447: F, t7475: F, t7478: F, t7485: F, t7486: F, t7491: F, t7493: F, t7494: F, t7504: F) -> (F, F, F) {
    let t7511 = t7510 * t261;
    let t7516 = F::cast_from(0.34246666666666666666e-1_f64) * t7335;
    let t7518 = -t5812 + F::cast_from(0.45662222222222222222e-1_f64) * t5522 - F::cast_from(0.17123333333333333333e-1_f64) * t5525 + F::cast_from(0.22831111111111111111e-1_f64) * t7357 - t7516 + F::new(0.5137e-1) * t7352;
    let t7521 = -t7446 + F::new(2.0) * t7447 * t704 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t7475 + F::cast_from(0.11696447245269292414e1_f64) * t7478 * t723 + F::cast_from(0.5848223622634646207e0_f64) * t2829 * t1972 + t7485 - F::new(2.0) * t7486 * t1918 - t7491 - t7493 - F::cast_from(0.11696447245269292414e1_f64) * t7494 * t1957 + t7504 - F::cast_from(0.19751673498613801407e-1_f64) * t7511 - F::new(0.310907e-1) * t7518 * t248;
    (t7511, t7518, t7521)
}
