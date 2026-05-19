//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1211/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1211<F: Float>(t38145: F, t6093: F, t9246: F, t2201: F, t3216: F, t3319: F, t3320: F, t10698: F, t12523: F, t39958: F, t39963: F, t39965: F, t39968: F, t39969: F, t39980: F, t39983: F, t39985: F, t41641: F) -> F {
    let t43447 = t6093 * t38145 * t9246;
    let t43451 = t2201 * t3319 * t3320 * t3216;
    let t43454 = t10698 * t12523;
    let t43457 = F::cast_from(0.13972381860938637374e0_f64) * t43447 - F::cast_from(0.23287303101564395623e-1_f64) * t43451 + F::cast_from(0.93149212406257582492e-1_f64) * t39958 + t39963 + F::cast_from(0.64025200389650807209e-1_f64) * t43454 + t39965 + t39968 + F::cast_from(0.14282990759302185292e-1_f64) * t39969 - t41641 - t39980 - t39983 + t39985;
    t43457
}
