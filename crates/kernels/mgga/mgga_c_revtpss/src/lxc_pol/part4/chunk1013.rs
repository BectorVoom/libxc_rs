//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1013/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1013<F: Float>(t9375: F, t177: F, t5566: F, t762: F, t1450: F, t5778: F, t3889: F, t5537: F, t1353: F, t13583: F, t13585: F, t13586: F, t1868: F, t3829: F, t4139: F, t5532: F, t5536: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t9547: F, t9599: F) -> (F, F, F) {
    let t13593 = 0.11696447245269292414e1 * t9375;
    let t13597 = t5566 * t177;
    let t13599 = 0.11696447245269292414e1 * t13597 * t762;
    let t13600 = t5778 * t1450;
    let t13607 = t5537 * t3889;
    let t13610 = 12.0 * t1353 * t13586 * t5536 + 6.0 * t1353 * t13600 * t4139 + 3.0 * t1868 * t4139 * t9547 - 3.0 * t1868 * t4139 * t9599 + 6.0 * t3829 * t5532 * t5536 + 6.0 * t13607 * t5536 + t13583 + t13585 - t13593 - t13599 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t9389 - t9391;
    (t13593, t13599, t13610)
}
