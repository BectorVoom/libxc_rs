//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 848/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk848<F: Float>(t34524: F, t34534: F, t103: F, t5710: F, t6557: F, t1332: F, t26061: F, t32545: F, t979: F, t1286: F, t31997: F, t32000: F, t32025: F, t32401: F, t34354: F, t34358: F, t34362: F, t34366: F, t34368: F, t34512: F, t34514: F, t6414: F, t6423: F, t6461: F, t7162: F, t7168: F) -> (F, F, F, F, F, F) {
    let t34535 = t34524 + t34534;
    let t34536 = t34535 * t103;
    let t34542 = t5710 * t6557;
    let t34544 = t26061 * t1332;
    let t34546 = t32545 * t979;
    let t34548 = -t31997 - t32000 - t32025 - t7162 * t6423 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1286 * t34354 - t1286 * t34358 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1286 * t34362 + t1286 * t34366 - F::new(2.0) * t34368 - F::new(2.0) * t34512 + F::new(4.0) * t34514 + F::new(2.0) * t34536 + t32401 - t6414 * t7168 / F::new(3.0) + t7162 * t6461 / F::new(6.0) - F::new(4.0) * t34542 - F::new(4.0) * t34544 - F::new(2.0) * t34546;
    (t34535, t34536, t34542, t34544, t34546, t34548)
}
