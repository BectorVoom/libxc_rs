//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2776/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2776(t14900: f64, t14923: f64, t10811: f64, t14914: f64, t14788: f64, t10886: f64, t14652: f64, t808: f64, t10489: f64, t10770: f64, t10818: f64, t14676: f64, t14791: f64, t14917: f64, t1544: f64, t2430: f64, t2477: f64, t2745: f64, t2749: f64, t40673: f64, t4343: f64, t4364: f64, t4450: f64, t50459: f64, t50560: f64, t50916: f64, t50933: f64, t50937: f64, t50939: f64, t50941: f64, t50943: f64, t50947: f64, t50955: f64, t50957: f64, t825: f64, t827: f64, t828: f64, t837: f64, t851: f64) -> f64 {
    let t50966 = t14923 * t14900;
    let t50968 = t10811 * t14914;
    let t50974 = t10811 * t14788;
    let t50977 = t10886 * t808 * t14652;
    let t50978 = 0.30492001685571196935e-4_f64 * t50977;
    let t50979 = -0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t50916 + 0.12862205435420921092e-1_f64 * t851 * t2477 * t828 * t4343 * t2430 + 0.42874018118069736972e-2_f64 * t851 * t2477 * t828 * t1544 * t10489 + 0.76230004213927992338e-3_f64 * t50933 + 0.21437009059034868486e-4_f64 * t50937 + 0.91464571985215438874e-3_f64 * t50939 + 455.0_f64 / 648.0_f64 * t50941 - 0.18295201011342718161e-3_f64 * t50943 - 0.15246000842785598468e-2_f64 * t50947 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t50560 * t2749 + t50955 - 0.77173232612525526549e-1_f64 * t50957 * t40673 * t4450 * t10818 - 0.64311027177104605458e-3_f64 * t2745 * t4364 * t50459 * t837 + 0.24009450146119052704e-1_f64 * t50966 + 0.30011812682648815881e-2_f64 * t50968 - 0.12862205435420921092e-1_f64 * t2745 * t10770 * t14676 * t14917 + 0.12004725073059526352e0_f64 * t50974 + t50978;
    t50979
}
