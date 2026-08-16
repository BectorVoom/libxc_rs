//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2316/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2316(t21038: f64, t225: f64, t10110: f64, t1527: f64, t1528: f64, t17049: f64, t17057: f64, t17064: f64, t17092: f64, t21013: f64, t21049: f64, t21054: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t40890: f64, t4147: f64, t4273: f64, t4300: f64, t5636: f64, t5657: f64, t59466: f64, t59537: f64, t798: f64, t855: f64, t865: f64, t866: f64) -> f64 {
    let t67305 = t21038 * t225;
    let t67322 = -18.0_f64 * t10110 * t4300 * t5636 * t855 + 6.0_f64 * t1527 * t17049 * t2718 * t855 + 24.0_f64 * t21049 * t40890 * t855 * t865 + 6.0_f64 * t2718 * t4300 * t5657 * t855 + t21013 * t259 * t798 - 3.0_f64 * t1528 * t59466 - 3.0_f64 * t1528 * t59537 + 6.0_f64 * t17057 * t4147 - 18.0_f64 * t17064 * t4147 + 12.0_f64 * t17092 * t4273 + 6.0_f64 * t21054 * t2597 + 6.0_f64 * t21054 * t2713 - 3.0_f64 * t67305 * t866;
    t67322
}
