//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 983/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk983<F: Float>(t1342: F, t8079: F, t2206: F, t3557: F, t2215: F, t10514: F, t10518: F, t10520: F, t10521: F, t10522: F, t10523: F, t10524: F, t10526: F, t10528: F, t10552: F, t198: F, t2439: F, t3728: F, t740: F, t7954: F, t7960: F, t7972: F, t7975: F, t8112: F, t8117: F, t8126: F) -> (F, F, F, F) {
    let t10557 = F::cast_from(4.0_f64) * t8079 * t1342;
    let t10558 = t3557 * t2206;
    let t10559 = F::cast_from(0.5848223622634646207e0_f64) * t10558;
    let t10560 = t3557 * t2215;
    let t10561 = F::cast_from(0.17315859105681463759e2_f64) * t10560;
    let t10562 = -F::cast_from(6.0_f64) * t10514 * t2439 * t3728 + F::cast_from(3.0_f64) * t10552 * t198 * t740 + t10518 + t10520 + t10521 + t10522 + t10523 - t10524 + t10526 + t10528 + t10557 - t10559 - t10561 - t7954 - t7960 + t7972 + t7975 + t8112 - t8117 - t8126;
    (t10557, t10559, t10561, t10562)
}
