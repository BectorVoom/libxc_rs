//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2631/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2631(t15419: f64, t21745: f64, t3447: f64, t20234: f64, t44505: f64, t1171: f64, t22104: f64, t15313: f64, t15320: f64, t18409: f64, t18416: f64, t4904: f64, t4919: f64, t4920: f64, t64756: f64, t64775: f64, t64811: f64, t65035: f64, t65041: f64, t65093: f64, t65112: f64, t65126: f64) -> (f64, f64, f64) {
    let t73491 = t3447 * t15419 * t21745;
    let t73496 = t44505 * t20234;
    let t73523 = t22104 * t1171;
    let t73525 = 0.8148148148148148148e-2_f64 * t64811 * t4920 + 0.83333333333333333331e-3_f64 * t3447 * t64775 * t4904 + 0.83333333333333333331e-3_f64 * t3447 * t18416 * t15313 + 0.83333333333333333331e-3_f64 * t3447 * t15320 * t18409 + 0.83333333333333333331e-3_f64 * t3447 * t4919 * t64756 - 0.16666666666666666666e-2_f64 * t65035 - 0.83333333333333333331e-3_f64 * t65041 + 0.44444444444444444443e-2_f64 * t65093 + 0.37037037037037037036e-3_f64 * t65112 - 0.24691358024691358024e-3_f64 * t65126 - 0.12674897119341563786e-1_f64 * t73523;
    (t73491, t73496, t73525)
}
