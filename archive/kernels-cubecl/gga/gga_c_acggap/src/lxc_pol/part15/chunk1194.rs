//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1194/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1194<F: Float>(t2176: F, t5511: F, t39743: F, t7942: F, t8306: F, t2146: F, t2395: F, t32329: F, t32990: F, t32997: F, t33000: F, t33566: F, t38008: F, t38010: F, t38015: F, t38018: F, t38019: F, t38092: F, t463: F, t7912: F, t7931: F, t8004: F, t8440: F, t9982: F, t9985: F) -> F {
    let t41044 = t2176 * t5511;
    let t41055 = t7942 * t8306 * t39743;
    let t41065 = -F::cast_from(0.34694512752820797848e1_f64) * t38008 + F::cast_from(0.65854491829355115987e0_f64) * t41044 - F::cast_from(0.17347256376410398924e1_f64) * t38010 - F::cast_from(0.8673628188205199462e0_f64) * t32329 + t38015 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t38092 * t8440 - F::cast_from(0.8673628188205199462e0_f64) * t7912 * t9982 + t38018 + F::cast_from(0.26341796731742046394e1_f64) * t38019 - F::cast_from(0.8673628188205199462e0_f64) * t41055 - F::cast_from(0.17347256376410398924e1_f64) * t32990 + t32997 + F::cast_from(0.52041769129231196772e1_f64) * t33000 + F::cast_from(0.17347256376410398924e1_f64) * t33566 * t2395 - F::cast_from(0.52041769129231196772e1_f64) * t2146 * t8004 * t9985 * t463;
    t41065
}
