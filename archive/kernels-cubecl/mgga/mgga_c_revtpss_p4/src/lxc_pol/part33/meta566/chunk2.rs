//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1969/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1969<F: Float>(t225: F, t30840: F, t494: F, t1794: F, t8201: F, t1287: F, t8197: F, t3783: F, t6628: F, t7660: F, t1770: F, t2144: F, t26889: F, t26895: F, t26906: F, t26922: F, t26949: F, t29136: F, t29141: F, t29275: F, t30736: F, t30740: F, t30744: F, t30748: F, t30752: F, t30758: F, t30764: F, t30768: F, t30772: F, t460: F, t6564: F, t7636: F, t7643: F, t7651: F, t8192: F, t8198: F, t8202: F, t8205: F, t8209: F, t8217: F) -> (F, F, F, F, F, F, F) {
    let t30842 = t30840 * t225 * t494;
    let t30849 = t8201 * t1794;
    let t30850 = t30849 * t1287;
    let t30853 = t8197 * t1794;
    let t30854 = t30853 * t1287;
    let t30860 = t7660 * t6628 * t3783;
    let t30865 = F::cast_from(0.8673628188205199462e0_f64) * t7643 * t30736 - F::cast_from(0.26020884564615598386e1_f64) * t26949 * t30740 - F::cast_from(0.17347256376410398924e1_f64) * t7636 * t30744 + F::cast_from(0.17347256376410398924e1_f64) * t7643 * t30748 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t30752 + F::cast_from(0.17347256376410398924e1_f64) * t29136 * t8202 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t30758 + F::cast_from(0.13170898365871023197e1_f64) * t1770 * t8192 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t30764 - F::cast_from(0.26020884564615598386e1_f64) * t7651 * t30768 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t30772 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t30842 + F::cast_from(0.65854491829355115987e0_f64) * t6564 * t2144 - F::cast_from(0.17347256376410398924e1_f64) * t29275 * t8198 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t30850 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t30854 - F::cast_from(0.8673628188205199462e0_f64) * t8205 * t8217 + F::cast_from(0.4336814094102599731e0_f64) * t26906 * t30860 + F::cast_from(0.17347256376410398924e1_f64) * t29141 * t8209;
    (t30842, t30849, t30850, t30853, t30854, t30860, t30865)
}
