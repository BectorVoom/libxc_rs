//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 602/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk602<F: Float>(t8417: F, t8431: F, t2347: F, t8234: F, t8236: F, t8238: F, t8242: F, t8245: F, t8249: F, t8253: F, t8257: F, t8261: F, t8263: F, t8265: F, t8269: F, t8272: F, t8276: F, t8280: F, t8284: F) -> (F, F, F) {
    let t8432 = t8417 + t8431;
    let t8436 = t2347 * t2347;
    let t8455 = F::new(0.9375e-1) * t8234 - F::new(0.1875e0) * t8236 + F::new(0.125e0) * t8238 + F::new(0.1875e0) * t8242 - F::new(0.125e0) * t8245 - F::new(0.9375e-1) * t8249 - F::cast_from(0.20833333333333333333e-1_f64) * t8253 + F::new(0.625e-1) * t8257 - F::cast_from(0.101171875e-1_f64) * t8261 + F::new(0.20234375e-1) * t8263 - F::cast_from(0.26979166666666666666e-1_f64) * t8265 - F::new(0.20234375e-1) * t8269 + F::cast_from(0.26979166666666666666e-1_f64) * t8272 + F::cast_from(0.101171875e-1_f64) * t8276 - F::cast_from(0.44965277777777777777e-2_f64) * t8280 - F::cast_from(0.13489583333333333333e-1_f64) * t8284;
    (t8432, t8436, t8455)
}
