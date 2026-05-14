//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1086/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1086<F: Float>(t30886: F, t7652: F, t1287: F, t1794: F, t29122: F, t2150: F, t30840: F, t473: F, t1828: F, t8197: F, t1775: F, t1829: F, t2149: F, t2152: F, t26906: F, t26976: F, t26994: F, t29129: F, t29207: F, t29220: F, t29227: F, t29304: F, t30867: F, t30870: F, t30874: F, t30878: F, t30883: F, t6574: F, t6580: F, t6588: F, t6703: F, t6745: F, t7602: F, t7632: F, t7636: F, t7651: F, t7659: F, t8213: F) -> (F, F, F, F, F) {
    let t30887 = t7652 * t30886;
    let t30893 = t29122 * t1794 * t1287;
    let t30899 = t2150 * t473 * t30840;
    let t30906 = t8197 * t1828;
    let t30907 = t7652 * t30906;
    let t30922 = 0.34694512752820797848e1 * t26994 * t30867 - 0.4336814094102599731e0 * t30870 * t2152 - 0.4336814094102599731e0 * t7659 * t30874 - 0.8673628188205199462e0 * t26906 * t30878 - 0.8673628188205199462e0 * t30883 * t2152 + 0.17347256376410398924e1 * t7651 * t30887 - 0.8673628188205199462e0 * t29129 * t8213 - 0.8673628188205199462e0 * t7659 * t30893 - 0.13170898365871023197e1 * t29227 * t1829 - 0.4336814094102599731e0 * t2149 * t30899 - 0.65854491829355115987e0 * t7632 * t6745 - 0.65854491829355115987e0 * t7602 * t6588 + 0.34694512752820797848e1 * t7636 * t30907 - 0.13170898365871023197e1 * t29207 * t1829 + 0.13170898365871023197e1 * t26976 * t6574 + 0.13170898365871023197e1 * t7632 * t6703 + 0.13170898365871023197e1 * t7602 * t6580 - 0.13170898365871023197e1 * t29304 * t1775 - 0.13170898365871023197e1 * t29220 * t1775;
    (t30887, t30893, t30899, t30907, t30922)
}
