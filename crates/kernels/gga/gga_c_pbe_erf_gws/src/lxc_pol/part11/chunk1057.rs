//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1057/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1057<F: Float>(t145: F, t169: F, t171: F, t18995: F, t18998: F, t19001: F, t19004: F, t19035: F, t19044: F, t242: F, t26031: F, t26034: F, t26038: F, t26051: F, t26061: F, t34237: F, t34244: F, t34254: F, t34274: F, t42876: F, t42880: F, t42891: F, t48321: F, t48520: F) -> (F,) {
    let t48908 = 0.533250677421793803e-1 * t145 * t48520 + 0.63671331549358746541e0 * t26061 - 0.16979021746495665744e1 * t26031 + t18995 + 0.63671331549358746541e0 * t34244 - 0.19101399464807623963e0 * t34254 - 0.4266005419374350424e0 * t42876 - t18998 - 0.51192065032492205088e1 * t26051 + t19001 - t19004 - 0.12734266309871749308e0 * t26034 - 0.12734266309871749308e0 * t42891 - 0.84895108732478328721e0 * t34237 + 0.20752137690161369243e1 * t26038 - 0.31835665774679373271e-1 * t169 * t171 * t48321 * t242 + t19035 - t19044 + 0.2122377718311958218e0 * t42880 + 0.19197024387184576908e1 * t34274;
    (t48908,)
}
