//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1347/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1347<F: Float>(t35966: F, t35970: F, t35973: F, t35976: F, t35979: F, t35983: F, t35986: F, t35989: F, t35992: F, t35996: F, t35999: F, t36003: F, t36006: F) -> F {
    let t36233 = F::cast_from(0.29357452990051769742e-5_f64) * t35966 - F::cast_from(0.46971924784082831588e-4_f64) * t35970 + F::cast_from(0.14678726495025884871e-5_f64) * t35973 - F::cast_from(0.18788769913633132635e-3_f64) * t35976 + F::cast_from(0.9785817663350589914e-7_f64) * t35979 + F::cast_from(0.66494999429992206259e-7_f64) * t35983 - F::cast_from(0.46971924784082831588e-5_f64) * t35986 - F::cast_from(0.46971924784082831588e-5_f64) * t35989 + F::cast_from(0.27838694088699758188e-6_f64) * t35992 + F::cast_from(0.30935748806067606286e-7_f64) * t35996 + F::cast_from(0.32829531147150437834e-4_f64) * t35999 - F::cast_from(0.17399183805437348867e-7_f64) * t36003 - F::cast_from(0.32829531147150437834e-4_f64) * t36006;
    t36233
}
