//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1362/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1362<F: Float>(t6628: F, t8208: F, t111832: F, t112121: F, t112129: F, t112714: F, t116293: F, t116323: F, t116327: F, t1287: F, t13143: F, t13149: F, t1774: F, t1794: F, t1829: F, t2142: F, t2149: F, t2150: F, t24525: F, t24543: F, t24633: F, t26889: F, t26895: F, t26907: F, t26922: F, t29141: F, t30752: F, t30771: F, t30887: F, t3769: F, t473: F, t6587: F, t7632: F, t7637: F, t7643: F, t7652: F, t8190: F, t8202: F, t96888: F, t96889: F, t97313: F) -> (F, F) {
    let t116500 = t8208 * t6628;
    let t116520 = -F::new(0.52041769129231196772e1) * t26889 * t112121 * t116327 + F::new(0.52041769129231196772e1) * t26895 * t112121 * t116323 + F::new(0.8673628188205199462e0) * t7643 * t7637 * t2142 * t24633 + F::new(0.26020884564615598386e1) * t7643 * t7637 * t8190 * t6587 - F::new(0.26020884564615598386e1) * t111832 * t30752 + F::new(0.52041769129231196772e1) * t112129 * t8202 + F::new(0.52041769129231196772e1) * t29141 * t30887 - F::new(0.4336814094102599731e0) * t2149 * t2150 * t473 * t116293 + F::new(0.26020884564615598386e1) * t26922 * t30771 * t1794 * t1287 + F::new(0.52041769129231196772e1) * t97313 * t116500 * t3769 - F::new(0.52041769129231196772e1) * t7643 * t7652 * t30771 * t1774 - F::new(0.39512695097613069591e1) * t7632 * t24525 - F::new(0.19756347548806534796e1) * t112714 * t1829 - F::new(0.26020884564615598386e1) * t96888 * t96889 * t24543 * t13149 + F::new(0.26020884564615598386e1) * t96888 * t26907 * t24543 * t13143;
    (t116500, t116520)
}
