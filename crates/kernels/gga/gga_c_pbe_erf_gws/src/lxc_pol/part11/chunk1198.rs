//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1198/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1198<F: Float>(t133: F, t19349: F, t19351: F, t2911: F, t3637: F, t3644: F, t42825: F, t42827: F, t48747: F, t48750: F, t48760: F, t48769: F, t48771: F, t48772: F, t48777: F, t48780: F, t48787: F, t48791: F, t48795: F, t8231: F) -> F {
    let t48856 = t48747 - t19349 + t19351 + t48750 + t48760 - t48769 + t48771 + t48772 - t48777 - t48780 + F::new(0.1034553e3) * t133 * t48787 - F::new(0.12414636e3) * t2911 * t8231 * t3644 * t3637 + F::new(0.15518295e2) * t133 * t48791 - F::new(0.1724255e1) * t133 * t48795 + F::cast_from(0.22990066666666666667e1_f64) * t42825 + F::new(0.2758808e2) * t42827;
    t48856
}
