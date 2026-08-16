//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 723/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk723(t225: f64, t3597: f64, t2131: f64, t23508: f64, t7325: f64, t3030: f64, t3502: f64, t478: f64, t1209: f64, t2141: f64, t3540: f64, t3: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24615 = t225 * t3597;
    let t24658 = t2131 * t23508;
    let t24659 = t24658 * t7325;
    let t24660 = t3030 * t3502;
    let t24661 = t24660 * t478;
    let t24667 = t3030 * t1209;
    let t24668 = t24667 * t478;
    let t24681 = t2141 * t3540 / 6912.0_f64;
    let t24682 = t7324 * t3;
    (t24615, t24659, t24661, t24668, t24681, t24682)
}
