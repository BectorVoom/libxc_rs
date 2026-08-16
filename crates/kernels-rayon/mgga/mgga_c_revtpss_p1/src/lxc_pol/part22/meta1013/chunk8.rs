//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3487/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3487(t1042: f64, t1047: f64, t11656: f64, t15707: f64, t15830: f64, t16167: f64, t16172: f64, t19792: f64, t19934: f64, t19940: f64, t3106: f64, t3161: f64, t3162: f64, t42371: f64, t4808: f64, t6312: f64, t65482: f64, t65596: f64, t65598: f64, t65610: f64, t65613: f64, t65618: f64) -> f64 {
    let t65626 = 0.6351706387862183255e-4_f64 * t65596 + 0.57165357490759649296e-3_f64 * t65598 + 0.30488190661738479624e-2_f64 * t11656 * t19940 - 0.42874018118069736972e-3_f64 * t3161 * t1042 * t65482 * t3162 + 0.60976381323476959249e-2_f64 * t3106 * t19934 - 0.5081365110289746604e-2_f64 * t15830 * t4808 + 0.6351706387862183255e-3_f64 * t65610 + 0.42874018118069736972e-3_f64 * t65613 * t1047 + 0.22866142996303859718e-2_f64 * t42371 * t6312 - 0.28582678745379824648e-3_f64 * t65618 - 0.28582678745379824648e-3_f64 * t15707 * t16167 - 0.47637797908966374413e-3_f64 * t15707 * t16172 + 0.30488190661738479624e-2_f64 * t11656 * t19792;
    t65626
}
