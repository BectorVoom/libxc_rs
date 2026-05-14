//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 856/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk856<F: Float>(t4719: F, t6219: F, t15101: F, t6110: F, t23466: F, t935: F, t2924: F, t19467: F, t4711: F, t981: F, t1699: F, t6400: F, t1079: F, t1695: F, t6244: F, t11133: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F, F, F, F, F, F) {
    let t23562 = 0.35089341735807877242e1 * t4719 * t6219;
    let t23564 = 6.0 * t15101 * t6110;
    let t23565 = t23466 * t935;
    let t23567 = 6.0 * t2924 * t23565;
    let t23568 = t19467 * t4711;
    let t23570 = 0.51947577317044391277e2 * t981 * t23568;
    let t23571 = t6400 * t1699;
    let t23583 = t1079 * t6244 * t1695;
    let t23598 = -t11133 - 0.19755555555555555556e-1 * t15189 + 0.9877777777777777778e-2 * t18919 - 0.29633333333333333334e-1 * t18924 + 0.14816666666666666667e-1 * t18934 - 0.16462962962962962963e-1 * t23479 + 0.59266666666666666668e-1 * t23483 - 0.29633333333333333334e-1 * t23501 - 0.88900000000000000002e-1 * t23487 + 0.88900000000000000002e-1 * t23505 - 0.14816666666666666667e-1 * t23490;
    (t23562, t23564, t23567, t23570, t23571, t23583, t23598)
}
