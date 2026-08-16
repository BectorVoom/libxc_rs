//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1134/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1134<F: Float>(t1127: F, t13421: F, t13422: F, t13491: F, t17847: F, t17877: F, t17890: F, t17993: F, t18133: F, t21250: F, t21282: F, t21325: F, t224: F, t2379: F, t238: F, t2384: F, t2387: F, t2394: F, t27704: F, t3759: F, t3766: F, t3789: F, t41621: F, t5025: F, t5049: F, t66667: F, t6758: F, t79341: F, t79373: F, t79559: F, t79593: F, t79997: F, t88503: F, t88813: F, t88858: F, t88881: F, t88891: F, t88898: F, t88911: F, t88916: F, t9524: F) -> F {
    let t88935 = -F::cast_from(0.24335568811288499135e-3_f64) * t17877 * t18133 + F::cast_from(0.27039520901431665705e-3_f64) * t17847 * t79997 - F::cast_from(0.38995437477448399246e-5_f64) * t3789 * t21250 * t88881 - F::cast_from(8.0_f64) * t3766 * t79559 * t1127 - F::cast_from(12.0_f64) * t3766 * t17890 * t5049 + F::cast_from(0.23238868087529279928e-2_f64) * t3759 * t2379 * t88891 + F::cast_from(0.73006706433865497404e-4_f64) * t41621 * t88503 * t2384 + F::cast_from(48.0_f64) * t3766 * t13491 * t88898 + F::cast_from(0.14225713554822031171e0_f64) * t224 * t13421 * t88858 - F::cast_from(0.93019603785751168e-1_f64) * t3759 * t79593 * t6758 + F::cast_from(0.43019436846165064134e-1_f64) * t238 * t88911 + F::cast_from(0.16223712540858999423e-2_f64) * t17847 * t79373 - F::cast_from(0.33728487690641211805e-2_f64) * t17993 * t88916 + F::cast_from(24.0_f64) * t3766 * t66667 * t5025 - F::cast_from(0.279058811357253504e0_f64) * t13422 * t21325 + F::cast_from(0.279058811357253504e-1_f64) * t3759 * t2394 * t88891 - F::cast_from(0.92955472350117119713e-3_f64) * t3759 * t9524 * t88813 + F::cast_from(0.46509801892875584e-1_f64) * t2387 * t79341 * t6758 - F::cast_from(0.279058811357253504e0_f64) * t27704 * t21282;
    t88935
}
