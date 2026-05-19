//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 800/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk800<F: Float>(t44: F, t2452: F, t406: F, t2267: F, t2625: F, t2858: F, t4904: F, t889: F, t1212: F, t35: F, t1216: F, t472: F, t1213: F, t1219: F, t2509: F, t2512: F, t40: F, t6980: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t7054 = t406 * t2452;
    let t7055 = F::new(8.0) * t7054;
    let t7057 = t2858 * t2267 * t2625;
    let t7058 = F::new(12.0) * t7057;
    let t7059 = t4904 * t889;
    let t7062 = t1212 * t35;
    let t7067 = t472 * t1216;
    let t7072 = piecewise3::<F>(t45, F::new(0.0), F::new(8.0) / F::new(27.0) * t7059 * t1213 - F::new(8.0) / F::new(9.0) * t7062 * t6980 - F::new(2.0) / F::new(9.0) * t2509 * t1219 + F::new(4.0) / F::new(3.0) * t7067 - F::new(4.0) * t2512 * t40);
    (t7055, t7058, t7072)
}
