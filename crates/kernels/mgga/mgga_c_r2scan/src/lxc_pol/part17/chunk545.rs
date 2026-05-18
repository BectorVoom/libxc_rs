//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 545/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk545<F: Float>(t322: F, t2940: F, t1348: F, t2983: F, t1338: F, t2952: F, t2954: F, t2982: F, t352: F, t855: F, t2464: F, t2486: F, t889: F) -> (F, F, F, F, F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t332 = F::new(0.25e1) < t322;
    let t2987 = piecewise3::<f64>(t332, t2940, F::new(0.0));
    let t2991 = t1348 * t2983;
    let t2995 = piecewise5::<f64>(t323, t2952 + t2954, t331, t2982, -F::new(0.21e1) * t1338 * t2983 * t352 - F::new(0.105e1) * t855 * t2987 * t352 - F::new(0.1575e1) * t2991 * t352);
    let t2997 = F::new(0.36622894612013090108e-3) * t2464;
    let t2998 = F::new(8.0) * t2486;
    let t2999 = t889 * t889;
    (t2987, t2991, t2995, t2997, t2998, t2999)
}
