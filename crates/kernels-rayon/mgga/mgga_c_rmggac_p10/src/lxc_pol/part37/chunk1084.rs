//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1084/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1084(t15872: f64, t4965: f64, t69954: f64, t71551: f64, t71552: f64, t71565: f64, t75678: f64, t77733: f64, t77737: f64, t77741: f64, t77745: f64, t77750: f64, t77755: f64, t77760: f64, t77765: f64, t77770: f64, t77772: f64, t77773: f64) -> f64 {
    let t80288 = t71551 - t71552 - t77733 + t77737 - t77741 - t77745 - t77750 + t77755 - t77760 - t77765 + t77770 + 0.39914139006212695214e-1_f64 * t4965 * t15872 - t69954 + t77772 - t71565 - t75678 + t77773;
    t80288
}
