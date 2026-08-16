//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1289/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1289<F: Float>(t25198: F, t7391: F, t3487: F, t739: F, t7803: F, t7805: F, t10939: F, t5694: F, t2617: F, t2963: F, t10834: F, t22883: F) -> (F, F, F, F, F) {
    let t33178 = t25198 * t7391;
    let t33179 = F::cast_from(0.89376224879626066674e-1_f64) * t33178;
    let t33182 = t7803 * t739 * t3487 * t7805;
    let t33183 = F::cast_from(0.76685851907841499352e0_f64) * t33182;
    let t33187 = F::cast_from(0.92686455430723328401e-1_f64) * t10939 * t5694;
    let t33193 = t7803 * t2963 * t2617;
    let t33194 = F::cast_from(0.38342925953920749676e0_f64) * t33193;
    let t33195 = t22883 * t10834;
    (t33179, t33183, t33187, t33194, t33195)
}
