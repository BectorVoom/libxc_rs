//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1145/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1145<F: Float>(t3382: F, t6249: F, t6245: F, t14072: F, t14081: F, t14086: F, t14091: F, t14096: F, t14101: F, t14105: F, t14107: F, t14109: F, t14111: F, t384: F, t398: F, t429: F, t6192: F) -> (F, F) {
    let t23618 = t3382 * t6249;
    let t23620 = t3382 * t6245;
    let t23630 = -0.85748036236139473944e-3 * t23618 + 0.85748036236139473944e-3 * t23620 + 0.10289764348336736874e-1 * t14072 - 0.51448821741683684367e-2 * t14081 + 0.51448821741683684367e-2 * t14086 - t14091 - 0.21437009059034868486e-3 * t14096 + 0.25724410870841842183e-2 * t14101 - t14105 - 0.13605355082800796533e0 * t14107 + 0.68026775414003982664e-1 * t14109 - 0.68026775414003982664e-1 * t14111;
    let t23636 = t384 * t398 * t429 * t6192;
    (t23630, t23636)
}
