//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 835/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk835<F: Float>(t3255: F, t5432: F, t5436: F, t5442: F, t11671: F, t544: F, t5428: F, t5454: F, t518: F, t5457: F, t5490: F, t1098: F, t5528: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16543 = F::new(0.13140859333333333334e-2) * t3255 * t5432;
    let t16545 = F::new(0.8760572888888888889e-3) * t3255 * t5436;
    let t16547 = F::new(0.17521145777777777778e-2) * t3255 * t5442;
    let t16552 = t11671 * t544;
    let t16562 = F::new(0.14600954814814814815e-2) * t3255 * t5428;
    let t16567 = F::new(0.13140859333333333333e-2) * t3255 * t5454;
    let t16582 = t5457 * t518;
    let t16587 = t3255 * t5490;
    let t16601 = F::new(0.13140859333333333333e-2) * t1098 * t5528;
    (t16543, t16545, t16547, t16552, t16562, t16567, t16582, t16587, t16601)
}
