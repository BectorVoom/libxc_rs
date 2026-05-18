//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1235/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1235<F: Float>(t12570: F, t792: F, t3262: F, t3276: F, t3275: F, t3582: F, t40705: F, t11519: F, t40282: F, t10918: F, t12391: F, t42846: F, t795: F) -> (F, F, F, F, F) {
    let t43729 = t12570 * t792;
    let t43732 = F::new(15.0) / F::new(16.0) * t3262 * t3276 * t43729;
    let t43735 = F::new(5.0) / F::new(8.0) * t3275 * t40705 * t3582;
    let t43739 = F::new(15.0) / F::new(8.0) * t40282 * t11519;
    let t43742 = F::new(3.0) / F::new(2.0) * t3262 * t10918 * t12391;
    let t43744 = t42846 * t795;
    (t43732, t43735, t43739, t43742, t43744)
}
