//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1027/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1027<F: Float>(t42487: F, t42491: F, t42494: F, t42496: F, t42499: F, t42501: F, t42503: F, t42506: F, t42509: F, t42512: F, t42514: F, t42516: F, t42518: F, t42520: F, t42523: F, t42904: F, t44244: F, t44246: F, t44250: F) -> F {
    let t51211 = t42487 + t42491 + t42494 + t42496 - t42499 + t42501 + t42503 + t42506 + t42509 + t42512 - t42514 - t42516 + t42518 - t42520 + t42523 + t42904 + t44250 - t44244 - t44246;
    t51211
}
