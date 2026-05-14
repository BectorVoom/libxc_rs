//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 850/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk850<F: Float>(t10020: F, t5829: F, t1215: F, t2579: F, t1580: F, t2487: F, t10025: F, t10182: F, t10184: F, t10187: F, t10190: F, t10193: F, t10199: F, t10201: F, t10204: F, t10206: F, t1629: F, t2587: F, t311: F, t5422: F, t5786: F, t5800: F, t5803: F, t5806: F, t5812: F, t5815: F, t5817: F) -> (F,) {
    let t10209 = t5829 * t10020;
    let t10216 = t2579 * t1215;
    let t10219 = t1580 * t2487;
    let t10222 = 0.14975624337724558 * t10182 + 0.02466859483068398 * t10184 - 0.02466859483068398 * t10187 - 0.14975624337724558 * t5422 + t10190 * t1629 / 6.0 - t10193 * t10025 / 3.0 + t5786 * t2587 / 6.0 + t10199 / 6.0 + t10201 * t1629 / 6.0 + t10204 / 6.0 + t10206 * t10025 / 3.0 - t10209 / 6.0 + t5800 / 6.0 - t5803 / 6.0 - t5806 / 6.0 - t5812 - t5815 / 12.0 + t5817 / 18.0 - t10216 * t311 / 6.0 - t10219 * t311 / 6.0;
    (t10222,)
}
