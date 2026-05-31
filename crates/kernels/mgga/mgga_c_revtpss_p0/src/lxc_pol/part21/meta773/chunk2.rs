//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2747/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2747<F: Float>(t14769: F, t2652: F, t10716: F, t14757: F, t14772: F, t221: F, t2674: F, t40683: F, t10698: F, t14494: F, t14785: F, t14917: F, t2394: F, t2745: F, t40503: F, t40507: F, t40509: F, t40511: F, t40518: F, t40523: F, t40526: F, t40529: F, t40532: F, t40535: F, t40549: F, t40553: F, t40558: F, t4343: F, t828: F, t851: F) -> F {
    let t50529 = t2652 * t14769;
    let t50531 = t10716 * t14757;
    let t50532 = F::cast_from(0.8131200449485652516e-2_f64) * t50531;
    let t50538 = t221 * t14772;
    let t50540 = t2674 * t40683 * t50538;
    let t50558 = -F::cast_from(0.60023625365297631762e-1_f64) * t50529 - t50532 - F::cast_from(0.77173232612525526549e-1_f64) * t851 * t10698 * t828 * t4343 * t2394 - F::cast_from(0.45738002528356795402e-2_f64) * t50540 + F::cast_from(0.85748036236139473944e-3_f64) * t40503 + t40507 + F::cast_from(0.76230004213927992336e-5_f64) * t40509 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t40511 - F::cast_from(0.13721400758507038621e-3_f64) * t40518 - F::cast_from(0.15246000842785598467e-4_f64) * t40523 - F::cast_from(0.54214778996945588148e-4_f64) * t40526 + F::cast_from(0.76230004213927992336e-5_f64) * t40529 + F::cast_from(0.27107389498472794074e-3_f64) * t40532 + F::cast_from(0.97586602194502058666e-3_f64) * t40535 + F::cast_from(0.42874018118069736972e-3_f64) * t40549 - F::cast_from(0.85748036236139473944e-4_f64) * t40553 - F::cast_from(0.85748036236139473944e-4_f64) * t40558 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t14494 * t14917;
    t50558
}
