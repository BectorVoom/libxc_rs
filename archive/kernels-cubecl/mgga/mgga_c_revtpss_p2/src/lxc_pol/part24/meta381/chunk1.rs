//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1281/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1281<F: Float>(t24792: F, t6429: F, t3626: F, t6425: F, t6421: F, t12787: F, t23842: F, t5268: F, t1042: F, t1261: F, t17448: F, t17605: F, t17792: F, t1782: F, t21213: F, t21283: F, t21285: F, t21287: F, t24787: F, t3625: F, t5373: F, t6640: F, t6659: F, t6663: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24793 = t6429 * t24792;
    let t24794 = t3626 * t24793;
    let t24797 = t6425 * t24792;
    let t24798 = t3626 * t24797;
    let t24803 = t6421 * t24792;
    let t24804 = t12787 * t24803;
    let t24807 = t5268 * t23842;
    let t24808 = t1042 * t24807;
    let t24815 = F::cast_from(0.42874018118069736972e-3_f64) * t21283 + F::cast_from(0.14481890564325777821e-1_f64) * t21285 - F::cast_from(0.45732285992607719436e-2_f64) * t21287 - F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t21213 * t1782 + t17792 / F::cast_from(54.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t24787 + F::cast_from(0.45732285992607719436e-2_f64) * t17605 * t6640 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t24794 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t24798 - F::cast_from(0.85748036236139473944e-3_f64) * t17448 * t6640 + F::cast_from(0.7145669686344956162e-3_f64) * t3625 * t24804 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t24808 + t5373 * t6659 / F::cast_from(36.0_f64) + t5373 * t6663 / F::cast_from(18.0_f64);
    (t24793, t24794, t24797, t24798, t24803, t24804, t24807, t24808, t24815)
}
