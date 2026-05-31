//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2828/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2828<F: Float>(t76401: F, t76419: F, t221: F, t23148: F, t2674: F, t2675: F, t14586: F, t14785: F, t14791: F, t1559: F, t18426: F, t18444: F, t18632: F, t2745: F, t4362: F, t4364: F, t4424: F, t4433: F, t6016: F, t6022: F, t61538: F, t61540: F, t61542: F, t61550: F, t61560: F, t61564: F, t61568: F, t61570: F, t61791: F, t76284: F, t76362: F, t76372: F, t775: F, t828: F, t837: F, t851: F, t855: F) -> (F, F) {
    let t76421 = t76401 / F::cast_from(2.0_f64) + t76419 / F::cast_from(2.0_f64);
    let t76428 = t2674 * t2675 * t221 * t23148;
    let t76434 = -F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t4364 * t18426 * t4424 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t76284 * t837 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t4364 * t61791 * t14586 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t4364 * t18444 * t18632 + F::cast_from(0.7623000421392799234e-4_f64) * t76362 - F::cast_from(0.6098400337114239387e-3_f64) * t61538 + F::cast_from(0.60023625365297631762e-1_f64) * t61540 - F::cast_from(0.12004725073059526352e-1_f64) * t61542 - F::cast_from(0.12004725073059526352e-1_f64) * t61550 + F::cast_from(0.25724410870841842184e-1_f64) * t4362 * t14785 * t6022 * t4433 - F::cast_from(0.10289764348336736873e-1_f64) * t4362 * t14791 * t14586 * t76372 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t1559 * t6016 * t775 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t855 * t828 * t76421 - F::cast_from(0.50820002809285328225e-4_f64) * t76428 - F::cast_from(0.42874018118069736972e-4_f64) * t61560 - F::cast_from(0.85748036236139473944e-4_f64) * t61564 - F::cast_from(0.85748036236139473944e-4_f64) * t61568 - F::cast_from(0.17006693853500995666e-1_f64) * t61570;
    (t76421, t76434)
}
