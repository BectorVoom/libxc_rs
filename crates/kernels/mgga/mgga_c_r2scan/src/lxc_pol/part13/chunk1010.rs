//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1010/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1010<F: Float>(t39499: F, t10697: F, t11669: F, t11671: F, t37619: F, t37656: F, t39482: F, t39485: F, t39487: F, t39490: F, t39492: F, t39493: F, t39494: F, t39495: F, t11670: F, t2124: F, t24454: F) -> (F, F) {
    let t39500 = 0.23115257973478049502e0 * t39499;
    let t39502 = t10697 * t11669 * t11671;
    let t39503 = 0.76830240467580968652e0 * t39502;
    let t39504 = 0.15573871527278325618e-1 * t39482 + 0.46721614581834976854e-1 * t39485 - t39487 + 0.11557628986739024751e0 * t37619 + 0.86682217400542685632e-1 * t39490 - t39492 - t39493 - t39494 + 0.10975748638225852664e0 * t39495 - 0.48787202696913915093e-2 * t37656 + t39500 - t39503;
    let t39506 = t11670 * t2124 * t24454;
    (t39504, t39506)
}
