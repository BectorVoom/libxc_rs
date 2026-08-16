//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2357/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2357(t21064: f64, t225: f64, t13042: f64, t13463: f64, t1528: f64, t17050: f64, t17052: f64, t17070: f64, t21034: f64, t252: f64, t259: f64, t2713: f64, t4142: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t5631: f64, t5637: f64, t5658: f64, t59503: f64, t68143: f64, t866: f64) -> f64 {
    let t68322 = t21064 * t225;
    let t68333 = t252 * t259 * t68143 + 3.0_f64 * t259 * t4142 * t5631 + 6.0_f64 * t13042 * t5637 + 6.0_f64 * t13463 * t5637 - 3.0_f64 * t13463 * t5658 - 3.0_f64 * t1528 * t59503 - 3.0_f64 * t17050 * t4147 + 6.0_f64 * t17052 * t4273 - 3.0_f64 * t17052 * t4301 + 12.0_f64 * t17070 * t4147 + 12.0_f64 * t17070 * t4268 - t21034 * t2713 - t68322 * t866;
    t68333
}
