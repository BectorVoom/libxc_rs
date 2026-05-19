//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1125/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1125<F: Float>(t10781: F, t7996: F, t10810: F, t574: F, t8066: F, t10697: F, t11669: F, t11671: F, t37619: F, t37656: F, t39482: F, t39485: F, t39487: F, t39490: F, t39492: F, t39493: F, t39494: F) -> F {
    let t39495 = t10781 * t7996;
    let t39499 = t574 * t10810 * t8066;
    let t39500 = F::cast_from(0.23115257973478049502e0_f64) * t39499;
    let t39502 = t10697 * t11669 * t11671;
    let t39503 = F::cast_from(0.76830240467580968652e0_f64) * t39502;
    let t39504 = F::cast_from(0.15573871527278325618e-1_f64) * t39482 + F::cast_from(0.46721614581834976854e-1_f64) * t39485 - t39487 + F::cast_from(0.11557628986739024751e0_f64) * t37619 + F::cast_from(0.86682217400542685632e-1_f64) * t39490 - t39492 - t39493 - t39494 + F::cast_from(0.10975748638225852664e0_f64) * t39495 - F::cast_from(0.48787202696913915093e-2_f64) * t37656 + t39500 - t39503;
    t39504
}
