//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1172/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1172<F: Float>(t2416: F, t4815: F, t4818: F, t10409: F, t1366: F, t16686: F, t16817: F, t2530: F, t2537: F, t4904: F, t4919: F, t4923: F, t50490: F, t56950: F, t56952: F, t56954: F, t56957: F, t57113: F, t57117: F, t57215: F, t57219: F, t57222: F, t57225: F, t57228: F, t57233: F, t7813: F) -> (F, F) {
    let t57260 = 36.0 * t2416 * t4815 * t4818;
    let t57275 = -t56950 - t56952 - t56954 - t56957 - t57113 - t57117 - t57215 + 0.21053604230838734656e2 * t2537 * t4904 * t4919 - t57219 + t57222 - t57225 - t57228 + t57233 + 0.2077890707925103596e3 * t10409 * t16686 - 0.62336721237753107879e3 * t7813 * t4923 * t4919 - 0.46785787179641632568e1 * t2530 * t16817 * t1366 + 0.69263023597503453196e2 * t2537 * t50490 * t1366;
    (t57260, t57275)
}
