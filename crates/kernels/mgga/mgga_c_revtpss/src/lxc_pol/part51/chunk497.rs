//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 497/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk497<F: Float>(t4533: F, t868: F, t1580: F, t213: F, t2437: F, t2443: F, t2446: F, t2449: F, t2460: F, t2462: F, t2468: F, t2473: F, t257: F, t2765: F, t4323: F, t4326: F, t4470: F, t4474: F, t4478: F, t4482: F, t4487: F, t865: F, t887: F) -> (F, F) {
    let t4534 = t868 * t4533;
    let t4537 = t2437 - t2443 - 0.54878743191129263322e-2 * t2446 + 0.54878743191129263322e-2 * t2449 + t2460 + 0.9757440539382783019e-2 * t2462 - 0.9757440539382783019e-2 * t2468 - t2473 - 0.54878743191129263322e-2 * t4323 + 0.9757440539382783019e-2 * t4326 + 0.65854491829355115987e0 * t213 * t4470 * t257 - 0.65854491829355115987e0 * t4474 * t887 + 0.54878743191129263322e-2 * t4478 - 0.9757440539382783019e-2 * t4482 - 0.65854491829355115987e0 * t2765 * t1580 + 0.13170898365871023197e1 * t865 * t4487 - 0.65854491829355115987e0 * t865 * t4534;
    (t4534, t4537)
}
