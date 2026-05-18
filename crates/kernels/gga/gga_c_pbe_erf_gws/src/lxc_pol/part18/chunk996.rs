//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 996/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk996<F: Float>(t10690: F, t10695: F, t10697: F, t10699: F, t10701: F, t10706: F, t10711: F, t10714: F, t10717: F, t10721: F, t10726: F, t10728: F, t10730: F, t10732: F, t10734: F, t10736: F, t7572: F) -> F {
    let t11212 = -t7572 - t10690 - t10695 - t10697 + t10699 + t10701 - t10706 - t10711 - t10714 + t10717 + t10721 + t10726 + t10728 - t10730 - t10732 + t10734 + t10736;
    t11212
}
