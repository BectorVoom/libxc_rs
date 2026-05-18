//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1278/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1278<F: Float>(t26264: F, t373: F, t2942: F, t2950: F, t2958: F, t1897: F, t1900: F, t8428: F, t11: F, t8620: F) -> (F, F, F, F, F, F, F) {
    let t26265 = F::new(0.13388493827160493828e1) * t26264;
    let t26266 = f64::powf(t373, -F::new(0.25e1));
    let t26267 = t2942 * t2942;
    let t26268 = t26266 * t26267;
    let t26270 = t2950 * t2950;
    let t26271 = t2958 * t26270;
    let t26276 = t8428 * t1897 * t1900;
    let t26278 = t11 * t8620 * t26276;
    (t26265, t26267, t26268, t26270, t26271, t26276, t26278)
}
