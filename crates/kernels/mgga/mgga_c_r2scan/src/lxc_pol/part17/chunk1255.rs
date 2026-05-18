//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1255/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1255<F: Float>(t38792: F, t38808: F, t40808: F, t40839: F, t40846: F, t41877: F, t41885: F, t41887: F, t42524: F, t42526: F, t42528: F, t42530: F, t42532: F, t42534: F, t42536: F, t42539: F, t42541: F, t42543: F) -> F {
    let t44641 = -F::new(3.0) / F::new(2.0) * t42524 + t42526 / F::new(2.0) + t42528 / F::new(4.0) + F::new(44.0) / F::new(9.0) * t40808 + t41877 + F::new(3.0) / F::new(2.0) * t42530 - t42532 - t42534 / F::new(2.0) - t42536 / F::new(4.0) + t40839 + F::new(4.0) / F::new(3.0) * t42539 + t38792 - t41885 + t41887 - F::new(2.0) / F::new(3.0) * t42541 + F::new(2.0) / F::new(3.0) * t42543 - t40846 + t38808;
    t44641
}
