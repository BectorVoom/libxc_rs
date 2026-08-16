//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1134/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1134(t1127: f64, t13421: f64, t13422: f64, t13491: f64, t17847: f64, t17877: f64, t17890: f64, t17993: f64, t18133: f64, t21250: f64, t21282: f64, t21325: f64, t224: f64, t2379: f64, t238: f64, t2384: f64, t2387: f64, t2394: f64, t27704: f64, t3759: f64, t3766: f64, t3789: f64, t41621: f64, t5025: f64, t5049: f64, t66667: f64, t6758: f64, t79341: f64, t79373: f64, t79559: f64, t79593: f64, t79997: f64, t88503: f64, t88813: f64, t88858: f64, t88881: f64, t88891: f64, t88898: f64, t88911: f64, t88916: f64, t9524: f64) -> f64 {
    let t88935 = -0.24335568811288499135e-3_f64 * t17877 * t18133 + 0.27039520901431665705e-3_f64 * t17847 * t79997 - 0.38995437477448399246e-5_f64 * t3789 * t21250 * t88881 - 8.0_f64 * t3766 * t79559 * t1127 - 12.0_f64 * t3766 * t17890 * t5049 + 0.23238868087529279928e-2_f64 * t3759 * t2379 * t88891 + 0.73006706433865497404e-4_f64 * t41621 * t88503 * t2384 + 48.0_f64 * t3766 * t13491 * t88898 + 0.14225713554822031171e0_f64 * t224 * t13421 * t88858 - 0.93019603785751168e-1_f64 * t3759 * t79593 * t6758 + 0.43019436846165064134e-1_f64 * t238 * t88911 + 0.16223712540858999423e-2_f64 * t17847 * t79373 - 0.33728487690641211805e-2_f64 * t17993 * t88916 + 24.0_f64 * t3766 * t66667 * t5025 - 0.279058811357253504e0_f64 * t13422 * t21325 + 0.279058811357253504e-1_f64 * t3759 * t2394 * t88891 - 0.92955472350117119713e-3_f64 * t3759 * t9524 * t88813 + 0.46509801892875584e-1_f64 * t2387 * t79341 * t6758 - 0.279058811357253504e0_f64 * t27704 * t21282;
    t88935
}
