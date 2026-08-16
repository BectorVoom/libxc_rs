//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1003/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1003(t125: f64, t3610: f64, t2175: f64, t783: f64, t2365: f64, t3676: f64, t3678: f64, t8313: f64, t2370: f64, t3628: f64, t3629: f64, t8306: f64, t8307: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10793 = t125 * t3610;
    let t10795 = t2175 * t10793 * t783;
    let t10799 = t2175 * t3676 * t2365;
    let t10803 = 7.0_f64 / 576.0_f64 * t8313 * t3678;
    let t10805 = t2175 * t3676 * t2370;
    let t10809 = t3628 * t3629 * t2370;
    let t10813 = t8306 * t3629 * t8307;
    (t10795, t10799, t10803, t10805, t10809, t10813)
}
