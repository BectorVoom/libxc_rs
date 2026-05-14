//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1401/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1401<F: Float>(t20922: F, t33636: F, t14294: F, t1520: F, t34846: F, t34888: F, t4165: F, t33640: F, t27431: F, t9483: F, t33630: F, t48680: F, t110120: F, t110136: F, t1620: F, t21345: F, t2347: F, t2748: F, t28036: F, t28046: F, t28053: F, t32523: F, t33708: F, t33743: F, t34906: F, t34919: F, t41849: F, t41861: F, t4535: F, t6604: F, t75337: F, t79120: F, t8436: F, t9560: F, t9571: F) -> (F, F, F, F, F, F, F) {
    let t120851 = 4.0 * t20922 * t33636;
    let t120854 = 12.0 * t14294 * t34846 * t1520;
    let t120855 = t4165 * t34888;
    let t120857 = 4.0 * t20922 * t33640;
    let t120877 = t9483 * t27431;
    let t120883 = 12.0 * t48680 * t33630;
    let t120886 = 2.0 * t1620 * t34906 * t4535 + 24.0 * t1620 * t34919 * t41849 + 4.0 * t2347 * t33743 * t4535 + 2.0 * t110120 * t8436 - 6.0 * t110136 * t28046 + 4.0 * t21345 * t33708 - t2748 * t79120 - t28036 * t9571 + 2.0 * t28053 * t32523 - 2.0 * t33743 * t6604 - 6.0 * t34919 * t41861 + 2.0 * t75337 * t9560 - t120851 + t120854 + t120855 - t120857 + t120877 + t120883;
    (t120851, t120854, t120855, t120857, t120877, t120883, t120886)
}
