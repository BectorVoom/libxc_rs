//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 831/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk831<F: Float>(t357: F, t359: F, t373: F, t9587: F, t1164: F, t3225: F, t334: F, t369: F, t86: F, t1143: F, t245: F, t238: F, t3419: F) -> (F, F, F, F, F, F) {
    let t10506 = F::cast_from(1.0_f64) / t359 / t357;
    let t10513 = t373 * t9587;
    let t10525 = t1164 * t3225;
    let t10541 = F::cast_from(0.11791604938271604938e-1_f64) * t86 * t334 * t369;
    let t10544 = t1143 * t245;
    let t10548 = t86 * t238 * t3419;
    (t10506, t10513, t10525, t10541, t10544, t10548)
}
