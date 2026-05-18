//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 662/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk662<F: Float>(t124: F, t383: F, t402: F, t625: F, t1460: F, t1478: F, t1482: F, t377: F, t1486: F, t1465: F, t1468: F, t1497: F) -> (F, F, F, F, F) {
    let t4788 = t124 * t383;
    let t4790 = t625 * t4788 * t402;
    let t4791 = F::new(0.71233333333333333332e-1) * t4790;
    let t4793 = t625 * t1460 * t1478;
    let t4794 = F::new(0.53424999999999999999e-1) * t4793;
    let t4795 = t377 * t1482;
    let t4797 = t625 * t4795 * t1486;
    let t4798 = F::new(0.85917975471764868594e0) * t4797;
    let t4805 = t625 * t377 * t1465 * t1468;
    let t4806 = F::new(0.10685e0) * t4805;
    let t4807 = t377 * t1497;
    (t4791, t4794, t4798, t4806, t4807)
}
