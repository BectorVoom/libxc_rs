//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2001/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2001<F: Float>(t1579: F, t26550: F, t103005: F, t25375: F, t2718: F, t7398: F, t26506: F, t27216: F, t14587: F, t25317: F, t25391: F, t25394: F, t27349: F, t27353: F, t2828: F, t28426: F, t28439: F, t28442: F, t7070: F, t8006: F, t92917: F, t93349: F, t95720: F, t95722: F, t95727: F, t95732: F, t95733: F, t95825: F, t99237: F) -> F {
    let t103037 = t26550 * t1579;
    let t103047 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t103005;
    let t103059 = t2718 * t7398;
    let t103063 = t27216 * t26506;
    let t103065 = -F::cast_from(0.14456046980341999104e-1_f64) * t95720 + F::cast_from(0.52041769129231196772e1_f64) * t93349 * t103037 * t25394 + F::cast_from(0.8673628188205199462e0_f64) * t99237 * t28439 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t95825 * t27349 - t103047 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t8006 * t2828 + F::cast_from(0.38549458614245330943e-1_f64) * t95722 - F::cast_from(0.68540937416128198418e-2_f64) * t95727 - t95732 + F::cast_from(0.25702851531048074406e-1_f64) * t95733 - F::cast_from(0.17347256376410398924e1_f64) * t92917 * t28442 - F::cast_from(0.17347256376410398924e1_f64) * t99237 * t28426 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t103059 * t14587 + F::cast_from(0.17135234354032049604e-1_f64) * t103063;
    t103065
}
