//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1303/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1303(t221: f64, t2675: f64, t4343: f64, t2674: f64, t243: f64, t4423: f64, t231: f64, t2662: f64, t2661: f64, t10722: f64, t1565: f64, t4352: f64, t4366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14857 = t2675 * t221 * t4343;
    let t14859 = 0.10164000561857065645e-3_f64 * t2674 * t14857;
    let t14860 = t243 * t4423;
    let t14861 = t14860 * t231;
    let t14862 = t2662 * t14861;
    let t14864 = 0.14291339372689912324e-4_f64 * t2661 * t14862;
    let t14866 = t10722 * t1565;
    let t14868 = t4352 * t4366;
    (t14857, t14859, t14861, t14864, t14866, t14868)
}
