//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 673/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk673<F: Float>(t234: F, t5006: F, t1520: F, t1531: F, t386: F, t518: F, t85: F, t462: F, t1510: F, t406: F, t1512: F, t410: F) -> (F, F, F, F, F, F) {
    let t5007 = t234 * t5006;
    let t5008 = F::new(0.10254018858216406658e4) * t5007;
    let t5015 = t1520 * t1531;
    let t5018 = t386 * t518 * t85;
    let t5019 = t462 * t5018;
    let t5020 = F::new(0.56968947174242584612e-3) * t5019;
    let t5021 = t406 * t1510;
    let t5025 = t406 * t1512;
    let t5027 = t410 * t1512;
    (t5008, t5015, t5020, t5021, t5025, t5027)
}
