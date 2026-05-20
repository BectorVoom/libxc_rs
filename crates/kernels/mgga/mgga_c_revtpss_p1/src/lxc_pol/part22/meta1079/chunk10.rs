//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3880/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3880<F: Float>(t124: F, t1370: F, t47199: F, t47216: F, t47229: F, t48945: F, t48947: F, t48951: F, t48955: F, t48971: F, t48975: F, t73578: F, t74547: F, t800: F) -> F {
    let t74558 = F::cast_from(0.50820002809285328224e-4_f64) * t48945 + F::cast_from(0.30234122406223992295e0_f64) * t48947 - F::cast_from(0.57165357490759649296e-3_f64) * t48951 - F::cast_from(0.28582678745379824648e-3_f64) * t48955 + F::new(7.0) / F::new(72.0) * t74547 - t1370 * t800 * t124 * t73578 / F::new(48.0) - F::cast_from(0.25692334753583138158e-2_f64) * t47199 - F::cast_from(0.27104001498285508386e-3_f64) * t47216 - F::cast_from(0.56688979511669985553e-2_f64) * t47229 - F::cast_from(0.16006300097412701803e-1_f64) * t48971 - F::cast_from(0.50820002809285328224e-4_f64) * t48975;
    t74558
}
