//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1342/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1342<F: Float>(t35966: F, t35970: F, t35973: F, t35976: F, t35979: F, t35983: F, t35986: F, t35989: F, t35992: F, t35996: F, t35999: F, t36003: F, t36006: F) -> F {
    let t36233 = F::new(0.29357452990051769742e-5) * t35966 - F::new(0.46971924784082831588e-4) * t35970 + F::new(0.14678726495025884871e-5) * t35973 - F::new(0.18788769913633132635e-3) * t35976 + F::new(0.9785817663350589914e-7) * t35979 + F::new(0.66494999429992206259e-7) * t35983 - F::new(0.46971924784082831588e-5) * t35986 - F::new(0.46971924784082831588e-5) * t35989 + F::new(0.27838694088699758188e-6) * t35992 + F::new(0.30935748806067606286e-7) * t35996 + F::new(0.32829531147150437834e-4) * t35999 - F::new(0.17399183805437348867e-7) * t36003 - F::new(0.32829531147150437834e-4) * t36006;
    t36233
}
