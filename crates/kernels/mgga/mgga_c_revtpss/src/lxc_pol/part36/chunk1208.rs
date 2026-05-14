//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1208/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1208<F: Float>(t116109: F, t116134: F, t116160: F, t116185: F, t116214: F, t116234: F, t116258: F, t116290: F, t1774: F, t5457: F, t1769: F, t111815: F, t111845: F, t111906: F, t1287: F, t1770: F, t1794: F, t2142: F, t21471: F, t225: F, t24515: F, t24998: F, t25015: F, t26889: F, t26895: F, t26922: F, t26994: F, t29194: F, t29200: F, t30771: F, t30842: F, t460: F, t494: F, t5464: F, t6563: F, t6587: F, t6622: F, t7602: F, t7636: F, t7637: F, t7651: F, t7652: F, t8190: F, t8197: F, t8208: F, t96979: F, t96986: F, t97308: F) -> (F, F, F, F) {
    let t116293 = t116109 + t116134 + t116160 + t116185 + t116214 + t116234 + t116258 + t116290;
    let t116323 = t5457 * t1774;
    let t116327 = t5457 * t1769;
    let t116331 = 0.52041769129231196772e1 * t7636 * t7652 * t30771 * t1769 + 0.19756347548806534796e1 * t1770 * t30842 + 0.8673628188205199462e0 * t7651 * t7652 * t2142 * t25015 + 0.52041769129231196772e1 * t26994 * t7637 * t8197 * t6587 - 0.26020884564615598386e1 * t7636 * t7637 * t8190 * t6563 + 0.65854491829355115987e0 * t460 * t116293 * t225 * t494 - 0.26020884564615598386e1 * t97308 * t111815 * t21471 * t1774 - 0.26020884564615598386e1 * t29194 * t111906 * t5464 * t1794 + 0.13010442282307799193e1 * t29200 * t111906 * t24998 - 0.52041769129231196772e1 * t96979 * t111815 * t5464 * t1769 + 0.26020884564615598386e1 * t26922 * t8208 * t6622 * t1287 + 0.19756347548806534796e1 * t7602 * t24515 + 0.52041769129231196772e1 * t96986 * t111815 * t5464 * t1774 + 0.26020884564615598386e1 * t26895 * t111845 * t116323 - 0.26020884564615598386e1 * t26889 * t111845 * t116327;
    (t116293, t116323, t116327, t116331)
}
