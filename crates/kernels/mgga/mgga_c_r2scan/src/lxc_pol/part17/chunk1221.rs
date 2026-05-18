//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1221/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1221<F: Float>(t39485: F, t39487: F, t39492: F, t39493: F, t39494: F, t41414: F, t41415: F, t41419: F, t41423: F, t43072: F, t43076: F, t43079: F) -> F {
    let t44216 = F::new(0.18688645832733990742e0) * t39485 - t39487 - t39492 - t39493 - t39494 - F::new(0.52396431978519890152e-1) * t43072 + t41414 - t41415 + t41419 - F::new(0.43663693315433241794e-2) * t43076 + F::new(0.46574606203128791246e-1) * t43079 + t41423;
    t44216
}
