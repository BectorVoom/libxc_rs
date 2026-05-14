//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 739/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk739<F: Float>(t7612: F, t880: F, t193: F, t1477: F, t6391: F, t1506: F, t6261: F, t681: F, t7617: F, t1466: F, t1479: F, t34090: F, t34114: F, t34173: F, t34212: F, t34231: F, t34262: F, t34267: F, t34276: F, t34278: F, t34283: F, t34312: F, t6210: F, t6263: F, t6267: F, t7581: F, t7587: F, t7618: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34321 = t7612 * t880;
    let t34322 = t193 * t34321;
    let t34325 = t1477 * t6391;
    let t34326 = t193 * t34325;
    let t34329 = t6261 * t1506;
    let t34330 = t193 * t34329;
    let t34333 = t681 * t7617;
    let t34335 = t1466 * t34333 / 9.0;
    let t34336 = t1466 * t34262 / 6.0 - t34267 + 8.0 * t34114 - 12.0 * t34173 + 8.0 * t34090 + 4.0 * t34231 - 4.0 * t34212 + t6210 * t7618 / 3.0 - t34276 - t1466 * t34278 / 3.0 + t34283 + t34312 * t1479 / 6.0 + t7581 * t6263 / 6.0 + t7581 * t6267 / 6.0 - t6210 * t7587 / 3.0 + t1466 * t34322 / 6.0 + t1466 * t34326 / 3.0 + t1466 * t34330 / 3.0 - t34335;
    (t34321, t34322, t34325, t34326, t34329, t34330, t34333, t34335, t34336)
}
