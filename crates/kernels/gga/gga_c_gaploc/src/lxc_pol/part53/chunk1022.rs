//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1022/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1022<F: Float>(t40234: F, t40237: F, t40245: F, t41968: F, t41972: F, t41975: F, t41976: F, t41978: F, t41979: F, t41980: F, t41981: F, t41982: F, t41983: F, t41984: F, t41987: F, t47987: F, t47989: F, t47995: F, t47997: F, t48001: F) -> F {
    let t50880 = -F::new(0.14300195980740170668e1) * t47987 - F::new(0.92023022289409799224e1) * t47989 + t41968 - t41972 - F::new(0.89376224879626066675e-1) * t40234 + F::new(0.59584149919750711115e-1) * t40237 - t41975 - t41976 - F::new(0.76685851907841499353e0) * t40245 + t41978 + t41979 - t41980 - t41981 + F::new(0.47667319935800568892e0) * t47995 - F::new(0.13803453343411469884e2) * t47997 - F::new(0.13803453343411469884e2) * t48001 + t41982 - t41983 + t41984 - t41987;
    t50880
}
