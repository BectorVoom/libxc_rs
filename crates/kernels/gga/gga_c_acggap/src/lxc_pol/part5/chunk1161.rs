//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1161/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1161<F: Float>(t1005: F, t5569: F, t1459: F, t1782: F, t384: F, t398: F, t879: F, t1008: F, t5878: F, t6076: F, t14341: F, t14343: F, t1524: F, t1579: F, t18628: F, t18633: F, t18647: F, t18649: F, t18651: F, t18653: F, t336: F, t367: F) -> (F,) {
    let t24051 = t1005 * t5569;
    let t24057 = t384 * t398 * t1459 * t1782 * t879;
    let t24064 = t1008 * t5878;
    let t24066 = t1008 * t6076;
    let t24072 = 0.32012600194825403606e-1 * t14341 + 0.16006300097412701803e-1 * t14343 - 0.25724410870841842184e-2 * t24051 - 0.34299214494455789578e-2 * t18628 - 0.12862205435420921092e-2 * t24057 + 0.51448821741683684366e-2 * t18633 - t367 * t336 * t1579 * t1524 / 24.0 - 0.34299214494455789578e-2 * t24064 - 0.17149607247227894789e-2 * t24066 + 0.32012600194825403606e-1 * t18647 + 0.16006300097412701803e-1 * t18649 + 0.16006300097412701803e-1 * t18651 - 0.80031500487063509016e-2 * t18653;
    (t24072,)
}
