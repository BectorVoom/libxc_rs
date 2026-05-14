//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1121/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1121<F: Float>(t11613: F, t2786: F, t996: F, t35966: F, t35970: F, t35973: F, t35976: F, t35979: F, t35983: F, t35986: F, t35989: F, t35992: F, t35996: F, t35999: F, t36003: F, t11616: F, t3212: F) -> (F, F) {
    let t36006 = t996 * t2786 * t11613;
    let t36008 = 0.14678726495025884871e-5 * t35966 - 0.23485962392041415794e-4 * t35970 + 0.73393632475129424356e-6 * t35973 - 0.93943849568165663176e-4 * t35976 + 0.4892908831675294957e-7 * t35979 + 0.3324749971499610313e-7 * t35983 - 0.23485962392041415794e-5 * t35986 - 0.23485962392041415794e-5 * t35989 + 0.13919347044349879094e-6 * t35992 + 0.15467874403033803143e-7 * t35996 + 0.16414765573575218917e-4 * t35999 - 0.86995919027186744337e-8 * t36003 - 0.16414765573575218917e-4 * t36006;
    let t36009 = t3212 * t11616;
    (t36008, t36009)
}
