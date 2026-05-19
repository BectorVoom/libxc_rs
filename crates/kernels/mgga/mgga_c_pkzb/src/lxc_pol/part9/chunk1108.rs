//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1108/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1108<F: Float>(t2312: F, t6233: F, t6110: F, t832: F, t2238: F, t338: F, t2241: F, t18439: F, t2189: F, t2239: F, t6198: F, t828: F) -> (F, F, F, F, F, F, F) {
    let t18580 = t6233 * t2312;
    let t18584 = t6110 * t832;
    let t18587 = t2238 * t2238;
    let t18589 = t338 / t18587;
    let t18591 = t2241 * t2241;
    let t18592 = F::new(1.0) / t18591;
    let t18596 = F::cast_from(0.96141975308641975307e-1_f64) * t18439;
    let t18609 = t2189 * t2239;
    let t18612 = t828 * t6198;
    (t18580, t18584, t18589, t18592, t18596, t18609, t18612)
}
