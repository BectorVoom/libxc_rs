//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 571/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk571<F: Float>(t2124: F, t321: F, t739: F, t446: F, t457: F, t201: F, t1979: F, t1982: F, t1162: F, t194: F, t1320: F, t1322: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7538 = t2124 * t321;
    let t7539 = t739 * t7538;
    let t7540 = F::new(0.11974241701863808564e0) * t7539;
    let t7541 = t446 * t457;
    let t7542 = t7541 * t201;
    let t7544 = t7542 * t1979 * t1982;
    let t7545 = F::new(0.85129199786595678796e-5) * t7544;
    let t7546 = t194 * t1162;
    let t7547 = t7546 * t201;
    let t7549 = t7547 * t1979 * t1982;
    let t7550 = F::new(0.42564599893297839398e-5) * t7549;
    let t7551 = t1320 * t1322;
    (t7538, t7540, t7541, t7542, t7545, t7546, t7547, t7550, t7551)
}
