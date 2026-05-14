//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 570/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk570<F: Float>(t1462: F, t236: F, t1971: F, t8517: F, t2344: F, t7494: F, t1587: F, t649: F, t27: F, t2134: F, t2329: F, t7501: F, t2084: F, t570: F, t2145: F, t551: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8518 = t236 * t1462;
    let t8519 = t1971 * t8518;
    let t8520 = t8517 * t8519;
    let t8523 = t7494 * t2344;
    let t8525 = t649 * t1587;
    let t8526 = t27 * t8525;
    let t8527 = t2134 * t8526;
    let t8529 = t7501 * t2329;
    let t8532 = t2084 * t570;
    let t8533 = t27 * t8532;
    let t8534 = t2145 * t8533;
    let t8536 = t2084 * t551;
    (t8519, t8520, t8523, t8526, t8527, t8529, t8533, t8534, t8536)
}
