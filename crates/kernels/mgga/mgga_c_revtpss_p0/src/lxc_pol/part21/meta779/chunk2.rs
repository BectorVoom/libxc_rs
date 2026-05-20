//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2776/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2776<F: Float>(t14900: F, t14923: F, t10811: F, t14914: F, t14788: F, t10886: F, t14652: F, t808: F, t10489: F, t10770: F, t10818: F, t14676: F, t14791: F, t14917: F, t1544: F, t2430: F, t2477: F, t2745: F, t2749: F, t40673: F, t4343: F, t4364: F, t4450: F, t50459: F, t50560: F, t50916: F, t50933: F, t50937: F, t50939: F, t50941: F, t50943: F, t50947: F, t50955: F, t50957: F, t825: F, t827: F, t828: F, t837: F, t851: F) -> F {
    let t50966 = t14923 * t14900;
    let t50968 = t10811 * t14914;
    let t50974 = t10811 * t14788;
    let t50977 = t10886 * t808 * t14652;
    let t50978 = F::cast_from(0.30492001685571196935e-4_f64) * t50977;
    let t50979 = -F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t50916 + F::cast_from(0.12862205435420921092e-1_f64) * t851 * t2477 * t828 * t4343 * t2430 + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t2477 * t828 * t1544 * t10489 + F::cast_from(0.76230004213927992338e-3_f64) * t50933 + F::cast_from(0.21437009059034868486e-4_f64) * t50937 + F::cast_from(0.91464571985215438874e-3_f64) * t50939 + F::new(455.0) / F::new(648.0) * t50941 - F::cast_from(0.18295201011342718161e-3_f64) * t50943 - F::cast_from(0.15246000842785598468e-2_f64) * t50947 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t50560 * t2749 + t50955 - F::cast_from(0.77173232612525526549e-1_f64) * t50957 * t40673 * t4450 * t10818 - F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t4364 * t50459 * t837 + F::cast_from(0.24009450146119052704e-1_f64) * t50966 + F::cast_from(0.30011812682648815881e-2_f64) * t50968 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t10770 * t14676 * t14917 + F::cast_from(0.12004725073059526352e0_f64) * t50974 + t50978;
    t50979
}
