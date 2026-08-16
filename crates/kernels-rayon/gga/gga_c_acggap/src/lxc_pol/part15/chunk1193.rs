//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1193/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1193(t301: f64, t642: f64, t1679: f64, t1717: f64, t9460: f64, t2138: f64, t2147: f64, t322: f64, t9985: f64, t157: f64, t1620: f64, t1937: f64, t2146: f64, t2152: f64, t2217: f64, t2222: f64, t32315: f64, t32324: f64, t36794: f64, t36808: f64, t36809: f64, t36811: f64, t40740: f64, t524: f64, t6558: f64, t7912: f64, t7931: f64, t8306: f64, t9003: f64, t9367: f64, t9391: f64, t9418: f64, t9977: f64) -> (f64, f64, f64) {
    let t41001 = t301 * t642;
    let t41006 = t1679 * t9460 * t1717;
    let t41027 = t2138 * t2147 * t9985 * t322;
    let t41042 = 0.8673628188205199462e0_f64 * t2146 * t2152 * t9367 * t524 * t157 + 0.17347256376410398924e1_f64 * t32315 + 0.17347256376410398924e1_f64 * t9003 * t9418 + t32324 + 0.10408353825846239354e2_f64 * t36794 - 0.26020884564615598386e1_f64 * t7912 * t9977 - 0.34694512752820797848e1_f64 * t41027 - 0.8673628188205199462e0_f64 * t7931 * t8306 * t40740 - 0.65854491829355115987e0_f64 * t2222 * t6558 - t36808 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t2217 * t1937 - 0.52041769129231196772e1_f64 * t36809 - 0.17347256376410398924e1_f64 * t36811 + 0.26341796731742046394e1_f64 * t9391 * t1620;
    (t41001, t41006, t41042)
}
