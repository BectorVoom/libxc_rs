//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1188/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1188<F: Float>(t35145: F, t35148: F, t30949: F, t30956: F, t30963: F, t30967: F, t30974: F, t30976: F, t30978: F, t30980: F, t30982: F, t30985: F, t30987: F, t35139: F, t35151: F, t35154: F, t35157: F) -> F {
    let t37408 = F::new(7.0) / F::new(36.0) * t35145;
    let t37409 = F::new(7.0) / F::new(36.0) * t35148;
    let t37419 = -F::new(0.32012600194825403606e-1) * t30949 - F::new(0.42874018118069736972e-3) * t35139 + F::new(0.42874018118069736972e-3) * t30956 + F::new(0.85748036236139473944e-3) * t30963 - F::new(0.14291339372689912324e-3) * t30967 + F::new(0.31448092289604152069e-3) * t30974 - t37408 - t37409 + t35151 / F::new(12.0) + t35154 / F::new(12.0) + t35157 / F::new(12.0) - F::new(0.64025200389650807212e-1) * t30976 + F::new(0.32012600194825403606e-1) * t30978 + F::new(0.32012600194825403606e-1) * t30980 - F::new(0.32012600194825403606e-1) * t30982 + F::new(0.42874018118069736972e-2) * t30985 - F::new(0.51448821741683684368e-2) * t30987;
    t37419
}
