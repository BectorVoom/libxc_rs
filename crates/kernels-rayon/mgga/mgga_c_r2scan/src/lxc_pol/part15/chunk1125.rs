//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1125/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1125(t10781: f64, t7996: f64, t10810: f64, t574: f64, t8066: f64, t10697: f64, t11669: f64, t11671: f64, t37619: f64, t37656: f64, t39482: f64, t39485: f64, t39487: f64, t39490: f64, t39492: f64, t39493: f64, t39494: f64) -> f64 {
    let t39495 = t10781 * t7996;
    let t39499 = t574 * t10810 * t8066;
    let t39500 = 0.23115257973478049502e0_f64 * t39499;
    let t39502 = t10697 * t11669 * t11671;
    let t39503 = 0.76830240467580968652e0_f64 * t39502;
    let t39504 = 0.15573871527278325618e-1_f64 * t39482 + 0.46721614581834976854e-1_f64 * t39485 - t39487 + 0.11557628986739024751e0_f64 * t37619 + 0.86682217400542685632e-1_f64 * t39490 - t39492 - t39493 - t39494 + 0.10975748638225852664e0_f64 * t39495 - 0.48787202696913915093e-2_f64 * t37656 + t39500 - t39503;
    t39504
}
