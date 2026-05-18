//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1153/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1153<F: Float>(t25049: F, t30955: F, t30957: F, t30962: F, t40954: F, t40956: F, t40958: F, t40960: F, t40989: F, t47931: F, t48006: F, t48034: F) -> F {
    let t48354 = F::new(0.18891666666666666667e-2) * t47931 + F::new(0.33585185185185185186e-2) * t30955 - F::new(0.25188888888888888889e-2) * t40954 + F::new(0.15113333333333333333e-1) * t40989 - F::new(0.78365432098765432099e-2) * t25049 + F::new(0.50377777777777777778e-2) * t30957 - F::new(0.2518888888888888889e-1) * t48034 + F::new(0.12594444444444444445e-1) * t48006 - F::new(0.27987654320987654323e-2) * t40956 + F::new(0.10075555555555555556e-1) * t40958 - F::new(0.15113333333333333333e-1) * t40960 - F::new(0.10075555555555555556e-1) * t30962;
    t48354
}
