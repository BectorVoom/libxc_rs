//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 993/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk993<F: Float>(t21251: F, t215: F, t4960: F, t5005: F, t1127: F, t5049: F, t207: F, t35382: F, t1690: F, t5010: F, t21172: F, t38176: F, t13421: F, t13422: F, t13491: F, t17847: F, t17877: F, t17890: F, t17993: F, t18133: F, t21250: F, t21282: F, t21325: F, t224: F, t2379: F, t238: F, t2384: F, t2387: F, t2394: F, t27704: F, t3759: F, t3766: F, t3789: F, t41621: F, t5025: F, t66667: F, t6758: F, t79341: F, t79373: F, t79559: F, t79593: F, t79997: F, t88503: F, t88813: F, t88858: F, t9524: F) -> (F, F, F, F) {
    let t88880 = t21251 * t215;
    let t88881 = 1.0 / t88880;
    let t88891 = t4960 * t5005;
    let t88898 = t5049 * t1127;
    let t88909 = 1.0 / t207 / t35382;
    let t88911 = t1690 * t5010 * t88909;
    let t88916 = t38176 * t21172;
    let t88935 = -0.24335568811288499135e-3 * t17877 * t18133 + 0.27039520901431665705e-3 * t17847 * t79997 - 0.38995437477448399246e-5 * t3789 * t21250 * t88881 - 8.0 * t3766 * t79559 * t1127 - 12.0 * t3766 * t17890 * t5049 + 0.23238868087529279928e-2 * t3759 * t2379 * t88891 + 0.73006706433865497404e-4 * t41621 * t88503 * t2384 + 48.0 * t3766 * t13491 * t88898 + 0.14225713554822031171e0 * t224 * t13421 * t88858 - 0.93019603785751168e-1 * t3759 * t79593 * t6758 + 0.43019436846165064134e-1 * t238 * t88911 + 0.16223712540858999423e-2 * t17847 * t79373 - 0.33728487690641211805e-2 * t17993 * t88916 + 24.0 * t3766 * t66667 * t5025 - 0.279058811357253504e0 * t13422 * t21325 + 0.279058811357253504e-1 * t3759 * t2394 * t88891 - 0.92955472350117119713e-3 * t3759 * t9524 * t88813 + 0.46509801892875584e-1 * t2387 * t79341 * t6758 - 0.279058811357253504e0 * t27704 * t21282;
    (t88881, t88909, t88911, t88935)
}
