//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1387/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1387<F: Float>(t2192: F, t9838: F, t18427: F, t18430: F, t18448: F, t18554: F, t18555: F, t27256: F, t27262: F, t27289: F, t27292: F, t27295: F, t27305: F, t18445: F, t18451: F, t22230: F, t22233: F, t22236: F, t27308: F, t27311: F, t27318: F, t27320: F, t27323: F, t27325: F, t27327: F) -> (F, F, F) {
    let t27850 = 2.0 * t2192 * t9838;
    let t27860 = 0.49294e0 * t27256 + t18554 - 0.18602370370370370371e1 * t18427 + 0.39862222222222222223e0 * t18430 + t18555 + 0.27385555555555555556e0 * t18448 - 0.59793333333333333334e0 * t27262 + 0.8969e0 * t27289 + 0.27385555555555555555e0 * t27292 + 0.39862222222222222223e0 * t27295 + 0.1898925e1 * t27305;
    let t27873 = 0.3071625e0 * t27308 + 0.3071625e0 * t27311 - 0.1460562962962962963e1 * t18445 + 0.27385555555555555556e0 * t18451 - 0.1860237037037037037e1 * t22230 + 0.15944888888888888889e1 * t22233 - 0.59793333333333333334e0 * t22236 + 0.142419375e1 * t27318 - 0.1898925e1 * t27320 - 0.1898925e1 * t27323 - 0.9494625e0 * t27325 - 0.76790625e-1 * t27327;
    (t27850, t27860, t27873)
}
