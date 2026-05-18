//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1164/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1164<F: Float>(t26950: F, t7637: F, t2142: F, t3584: F, t3551: F, t1204: F, t2144: F, t26886: F, t26889: F, t26891: F, t26895: F, t26897: F, t26901: F, t26906: F, t26909: F, t26913: F, t26918: F, t26922: F, t26924: F, t26928: F, t26933: F, t26937: F, t26941: F, t26945: F, t26949: F, t3552: F, t3791: F, t460: F, t7629: F, t7632: F, t7636: F, t7643: F, t7651: F, t7654: F, t7659: F, t7662: F) -> (F, F, F, F, F, F) {
    let t26951 = t7637 * t26950;
    let t26958 = t2142 * t3584;
    let t26959 = t7637 * t26958;
    let t26962 = t2142 * t3551;
    let t26963 = t7637 * t26962;
    let t26968 = F::new(0.65854491829355115987e0) * t460 * t26886 - F::new(0.17347256376410398924e1) * t26889 * t26891 + F::new(0.17347256376410398924e1) * t26895 * t26897 - F::new(0.4336814094102599731e0) * t7659 * t26901 - F::new(0.8673628188205199462e0) * t26906 * t26909 + F::new(0.4336814094102599731e0) * t26906 * t26913 - F::new(0.8673628188205199462e0) * t26918 * t7662 + F::new(0.17347256376410398924e1) * t26922 * t26924 + F::new(0.34694512752820797848e1) * t7636 * t26928 - F::new(0.8673628188205199462e0) * t7659 * t26933 + F::new(0.17347256376410398924e1) * t26937 * t7654 - F::new(0.17347256376410398924e1) * t7636 * t26941 + F::new(0.17347256376410398924e1) * t7651 * t26945 - F::new(0.26020884564615598386e1) * t26949 * t26951 + F::new(0.65854491829355115987e0) * t3552 * t2144 + F::new(0.13170898365871023197e1) * t1204 * t7629 + F::new(0.8673628188205199462e0) * t7643 * t26959 - F::new(0.8673628188205199462e0) * t7636 * t26963 - F::new(0.65854491829355115987e0) * t7632 * t3791;
    (t26951, t26958, t26959, t26962, t26963, t26968)
}
