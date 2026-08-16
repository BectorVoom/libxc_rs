//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1039/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1039<F: Float>(t10572: F, t1379: F, t3683: F, t10578: F, t10579: F, t4722: F, t4707: F, t750: F, t762: F, t1368: F, t3610: F, t4711: F) -> (F, F, F, F, F) {
    let t14322 = t10572 * t1379 * t3683;
    let t14326 = t10578 * t10579 * t4722;
    let t14330 = t762 * t4707 * t750;
    let t14334 = t762 * t1368 * t3610;
    let t14338 = t762 * t4711 * t750;
    (t14322, t14326, t14330, t14334, t14338)
}
