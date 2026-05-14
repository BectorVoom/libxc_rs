//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 995/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk995<F: Float>(t21040: F, t6638: F, t3626: F, t471: F, t5351: F, t6429: F, t6425: F, t6421: F, t12787: F, t23842: F, t5268: F, t1042: F, t1261: F, t17448: F, t17605: F, t17792: F, t1782: F, t21213: F, t21283: F, t21285: F, t21287: F, t3625: F, t5373: F, t6640: F, t6659: F, t6663: F) -> (F, F, F, F, F, F) {
    let t24786 = t21040 * t6638;
    let t24787 = t3626 * t24786;
    let t24792 = t5351 * t471;
    let t24793 = t6429 * t24792;
    let t24794 = t3626 * t24793;
    let t24797 = t6425 * t24792;
    let t24798 = t3626 * t24797;
    let t24803 = t6421 * t24792;
    let t24804 = t12787 * t24803;
    let t24807 = t5268 * t23842;
    let t24808 = t1042 * t24807;
    let t24815 = 0.42874018118069736972e-3 * t21283 + 0.14481890564325777821e-1 * t21285 - 0.45732285992607719436e-2 * t21287 - 11.0 / 108.0 * t21213 * t1782 + t17792 / 54.0 - 0.42874018118069736972e-3 * t3625 * t24787 + 0.45732285992607719436e-2 * t17605 * t6640 - 0.42874018118069736972e-3 * t3625 * t24794 - 0.85748036236139473944e-3 * t3625 * t24798 - 0.85748036236139473944e-3 * t17448 * t6640 + 0.7145669686344956162e-3 * t3625 * t24804 - 0.85748036236139473944e-3 * t1261 * t24808 + t5373 * t6659 / 36.0 + t5373 * t6663 / 18.0;
    (t24787, t24794, t24798, t24804, t24808, t24815)
}
