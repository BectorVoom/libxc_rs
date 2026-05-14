//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 633/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk633<F: Float>(t13506: F, t7226: F, t2508: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F, t471: F, t3611: F, t871: F, t12580: F, t13088: F, t13089: F) -> (F, F, F, F) {
    let t13507 = t7226 * t13506;
    let t13509 = 0.46143157380853345701e-1 * t2508 * t13507;
    let t13516 = -3.0 / 128.0 * t12555 - 27.0 / 4096.0 * t12558 + 27.0 / 262144.0 * t12561 - 9.0 / 262144.0 * t12564 + 9.0 / 4096.0 * t12566 + t12569 / 128.0;
    let t13517 = t13516 * t471;
    let t13518 = t3611 * t871;
    let t13520 = 9.0 / 128.0 * t12555;
    let t13521 = 9.0 / 4096.0 * t12558;
    let t13522 = 3.0 / 4096.0 * t12566;
    let t13523 = 3.0 / 128.0 * t12569;
    let t13524 = 4.0 * t12580;
    let t13525 = t13517 + t13518 / 2.0 + t13088 - t13089 - t13520 - t13521 + t13522 + t13523 + t13524;
    (t13507, t13509, t13516, t13525)
}
