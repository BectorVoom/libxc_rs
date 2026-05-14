//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 880/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk880<F: Float>(t10683: F, t11587: F, t10680: F, t10674: F, t897: F, t10676: F, t10673: F, t2810: F, t3446: F, t3453: F, t2813: F, t3308: F, t3692: F, t3429: F, t2816: F, t10660: F, t10678: F, t10685: F, t11517: F, t11521: F, t11525: F, t11527: F, t11530: F, t11580: F, t11585: F) -> (F, F, F, F, F, F, F, F) {
    let t11588 = t11587 * t10683;
    let t11589 = t10680 * t11588;
    let t11591 = t10674 * t897;
    let t11592 = t11591 * t10676;
    let t11593 = t10673 * t11592;
    let t11598 = t3446 * t3453 * t2810;
    let t11601 = t3446 * t3453 * t2813;
    let t11603 = t3308 * t3692;
    let t11604 = t3429 * t11603;
    let t11607 = t3446 * t3453 * t2816;
    let t11609 = -0.15243824895787514157e-3 * t10660 + 0.96056421943322389208e-3 * t11580 - t11517 + t11521 + 0.36021158228745895953e-3 * t11585 + 0.36021158228745895953e-3 * t11589 - 0.5124043883133942371e-4 * t11593 - 0.51240438831339423711e-4 * t10678 + 0.36021158228745895953e-3 * t10685 - 0.36021158228745895953e-3 * t11598 - 0.36021158228745895953e-3 * t11601 - 0.15243824895787514157e-3 * t11604 - 0.36021158228745895953e-3 * t11607 - t11525 - t11527 + t11530;
    (t11588, t11589, t11591, t11592, t11593, t11603, t11604, t11609)
}
