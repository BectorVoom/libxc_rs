//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1291/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1291<F: Float>(t12944: F, t12947: F, t12949: F, t12953: F, t12958: F, t12961: F, t12966: F, t41147: F, t41148: F, t41149: F, t41150: F, t42369: F, t42370: F, t42371: F, t42372: F, t42373: F) -> F {
    let t45116 = -t12944 + t12966 - t12947 + t12949 - t41147 + t41148 + t41149 + t41150 + t42369 + t42370 - t12953 + t42371 - t42372 - t42373 + t12958 - t12961;
    t45116
}
