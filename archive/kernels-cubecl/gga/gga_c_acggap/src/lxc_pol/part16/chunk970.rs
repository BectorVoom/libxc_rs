//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 970/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk970<F: Float>(t34361: F, t10146: F, t420: F, t576: F, t1083: F, t137: F, t1511: F, t2020: F, t7440: F, t8631: F, t2318: F, t31261: F) -> (F, F, F, F, F, F) {
    let t34362 = F::cast_from(0.12862205435420921092e-1_f64) * t34361;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34382 = t2020 * t1511;
    let t34383 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t34382;
    let t34390 = t7440 * t8631;
    let t34391 = F::cast_from(0.5603125e-1_f64) * t34390;
    let t34392 = t31261 * t2318;
    (t34362, t34368, t34369, t34383, t34391, t34392)
}
