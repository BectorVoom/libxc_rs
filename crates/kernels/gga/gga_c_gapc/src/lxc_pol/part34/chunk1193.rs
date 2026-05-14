//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1193/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1193<F: Float>(t34878: F, t34881: F, t34886: F, t34891: F, t34897: F, t34873: F, t34876: F, t34884: F, t34889: F, t34894: F, t34900: F, t34909: F, t34911: F, t34914: F, t34918: F, t34926: F) -> (F, F, F, F, F, F) {
    let t37111 = 0.11196959561581759181e-6 * t34878;
    let t37112 = 0.16555927416768851825e-5 * t34881;
    let t37114 = 0.28137654660407340486e-7 * t34886;
    let t37116 = 0.41268560168597432712e-4 * t34891;
    let t37118 = 0.2845640240200497334e-7 * t34897;
    let t37120 = -0.3623181683912940217e-6 * t34873 - 0.44979384805509945071e-8 * t34876 + t37111 + t37112 - 0.19666550313313802087e-7 * t34884 + t37114 - 0.52389984474979915324e-8 * t34889 - t37116 + 0.93149392396514289451e-9 * t34894 + t37118 - 0.505954834707648426e-7 * t34900;
    let t37124 = 0.16867947048611111112e-5 * t34909;
    let t37125 = 0.14759453667534722223e-5 * t34911;
    let t37126 = 0.14759453667534722223e-5 * t34914;
    let t37127 = 0.88465285289519332099e-6 * t34918;
    let t37129 = 0.23333993417245370372e-3 * t34926;
    (t37120, t37124, t37125, t37126, t37127, t37129)
}
