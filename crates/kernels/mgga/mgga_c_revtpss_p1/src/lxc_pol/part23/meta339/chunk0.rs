//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1641/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1641<F: Float>(t14239: F, t4104: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t1385: F, t5710: F, t1904: F, t3899: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t14241 = F::cast_from(0.19514881078765566038e-1_f64) * t14239 * t4104;
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14255 = t1385 * t5710;
    let t14274 = t3899 * t1904;
    let t14276 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14274;
    (t14241, t14242, t14243, t14252, t14255, t14274, t14276)
}
