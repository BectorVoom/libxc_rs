//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1399/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1399<F: Float>(t34878: F, t34881: F, t34886: F, t34891: F, t34897: F, t34909: F, t34911: F, t34914: F, t34918: F, t34926: F, t34929: F, t34934: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37111 = F::new(0.11196959561581759181e-6) * t34878;
    let t37112 = F::new(0.16555927416768851825e-5) * t34881;
    let t37114 = F::new(0.28137654660407340486e-7) * t34886;
    let t37116 = F::new(0.41268560168597432712e-4) * t34891;
    let t37118 = F::new(0.2845640240200497334e-7) * t34897;
    let t37124 = F::new(0.16867947048611111112e-5) * t34909;
    let t37125 = F::new(0.14759453667534722223e-5) * t34911;
    let t37126 = F::new(0.14759453667534722223e-5) * t34914;
    let t37127 = F::new(0.88465285289519332099e-6) * t34918;
    let t37129 = F::new(0.23333993417245370372e-3) * t34926;
    let t37130 = F::new(0.27012148473991046866e-5) * t34929;
    let t37131 = F::new(0.21915101773490614185e-6) * t34934;
    (t37111, t37112, t37114, t37116, t37118, t37124, t37125, t37126, t37127, t37129, t37130, t37131)
}
