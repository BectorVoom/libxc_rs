//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1067/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1067<F: Float>(t10698: F, t12523: F, t39958: F, t39963: F, t39965: F, t39968: F, t39969: F, t39980: F, t39983: F, t39985: F, t41641: F, t43447: F, t43451: F, t3602: F, t39922: F, t8081: F) -> (F, F) {
    let t43454 = t10698 * t12523;
    let t43457 = 0.13972381860938637374e0 * t43447 - 0.23287303101564395623e-1 * t43451 + 0.93149212406257582492e-1 * t39958 + t39963 + 0.64025200389650807209e-1 * t43454 + t39965 + t39968 + 0.14282990759302185292e-1 * t39969 - t41641 - t39980 - t39983 + t39985;
    let t43459 = t39922 * t3602 * t8081;
    (t43457, t43459)
}
