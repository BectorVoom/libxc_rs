//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2877/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2877(t41832: f64, t4732: f64, t981: f64, t11524: f64, t15525: f64, t11299: f64, t11300: f64, t1610: f64, t11112: f64, t15101: f64, t11116: f64, t15421: f64) -> (f64, f64, f64, f64, f64) {
    let t52201 = 0.17315859105681463759e2_f64 * t981 * t4732 * t41832;
    let t52204 = 0.51947577317044391277e2_f64 * t981 * t15525 * t11524;
    let t52207 = 24.0_f64 * t11299 * t1610 * t11300;
    let t52209 = 6.0_f64 * t15101 * t11112;
    let t52211 = 0.48245938496077605201e2_f64 * t15421 * t11116;
    (t52201, t52204, t52207, t52209, t52211)
}
