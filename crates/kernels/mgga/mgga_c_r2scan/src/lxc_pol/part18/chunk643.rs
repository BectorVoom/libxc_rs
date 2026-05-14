//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 643/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk643<F: Float>(t4704: F, t4997: F, t234: F, t446: F, t453: F, t4854: F, t4811: F, t4859: F, t4862: F, t1520: F, t1531: F, t386: F, t518: F, t85: F, t462: F, t1510: F, t406: F) -> (F, F, F, F, F, F) {
    let t4998 = t4997 * t4704;
    let t4999 = t234 * t4998;
    let t5000 = 0.51947577317044391277e2 * t4999;
    let t5002 = t446 * t4854 * t453;
    let t5003 = t234 * t5002;
    let t5004 = 0.5848223622634646207e0 * t5003;
    let t5006 = t4859 * t4811 * t4862;
    let t5007 = t234 * t5006;
    let t5008 = 0.10254018858216406658e4 * t5007;
    let t5015 = t1520 * t1531;
    let t5018 = t386 * t518 * t85;
    let t5019 = t462 * t5018;
    let t5020 = 0.56968947174242584612e-3 * t5019;
    let t5021 = t406 * t1510;
    (t5000, t5004, t5008, t5015, t5020, t5021)
}
