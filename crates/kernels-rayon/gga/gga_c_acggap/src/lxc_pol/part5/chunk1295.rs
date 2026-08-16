//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1295/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1295(t1005: f64, t5569: f64, t1459: f64, t1782: f64, t384: f64, t398: f64, t879: f64, t1008: f64, t5878: f64, t6076: f64, t14341: f64, t14343: f64, t1524: f64, t1579: f64, t18628: f64, t18633: f64, t18647: f64, t18649: f64, t18651: f64, t18653: f64, t336: f64, t367: f64) -> f64 {
    let t24051 = t1005 * t5569;
    let t24057 = t384 * t398 * t1459 * t1782 * t879;
    let t24064 = t1008 * t5878;
    let t24066 = t1008 * t6076;
    let t24072 = 0.32012600194825403606e-1_f64 * t14341 + 0.16006300097412701803e-1_f64 * t14343 - 0.25724410870841842184e-2_f64 * t24051 - 0.34299214494455789578e-2_f64 * t18628 - 0.12862205435420921092e-2_f64 * t24057 + 0.51448821741683684366e-2_f64 * t18633 - t367 * t336 * t1579 * t1524 / 24.0_f64 - 0.34299214494455789578e-2_f64 * t24064 - 0.17149607247227894789e-2_f64 * t24066 + 0.32012600194825403606e-1_f64 * t18647 + 0.16006300097412701803e-1_f64 * t18649 + 0.16006300097412701803e-1_f64 * t18651 - 0.80031500487063509016e-2_f64 * t18653;
    t24072
}
