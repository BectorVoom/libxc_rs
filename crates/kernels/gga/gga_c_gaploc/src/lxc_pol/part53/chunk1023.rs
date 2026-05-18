//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1023/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1023<F: Float>(t41989: F, t41991: F, t41992: F, t41996: F, t42005: F, t42008: F, t42018: F, t42022: F, t42029: F, t42047: F, t42051: F, t48011: F, t48013: F, t48017: F, t48020: F, t48023: F, t48026: F, t48029: F, t48034: F) -> F {
    let t50884 = -t41989 + t41991 + t41992 - t41996 + t42005 + t42008 - t42018 - t42022 + t48011 + t48013 + t48017 - t48020 + t48023 - t48026 + t48029 - t48034 + t42029 + t42047 + t42051;
    t50884
}
