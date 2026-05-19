//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 465/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk465<F: Float>(t43: F, t50: F, t2059: F, t2062: F, t1524: F, t1526: F, t1529: F, t1531: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t2063 = t2059 * t2062;
    let t2064 = F::cast_from(0.63272429661648472106e0_f64) * t2063;
    let t2068 = piecewise3::<F>(t44, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1524 + F::new(2.0) / F::new(3.0) * t1526);
    let t2072 = piecewise3::<F>(t51, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1529 + F::new(2.0) / F::new(3.0) * t1531);
    let t2074 = t2068 / F::new(2.0) + t2072 / F::new(2.0);
    (t2064, t2074)
}
