//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1229/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1229<F: Float>(t12227: F, t1413: F, t1449: F, t1450: F, t1466: F, t16047: F, t16074: F, t23708: F, t23711: F, t23716: F, t23779: F, t23819: F, t23855: F, t23887: F, t2481: F, t3337: F, t3340: F, t3356: F, t42: F, t4218: F, t430: F, t448: F, t453: F, t4769: F, t4772: F, t4828: F, t6631: F, t6634: F, t6642: F, t6692: F, t8599: F, t8607: F, t8610: F, t8661: F, t8705: F, t995: F) -> (F,) {
    let t23891 = -0.1325e0 * t6634 * t8607 - 0.1325e0 * t6634 * t8610 - 0.6625e-1 * t1413 * t6692 * t995 - 0.1325e0 * t12227 * t6642 + 0.99375e0 * t16047 * t3340 * t1450 + 0.99375e-1 * t4772 * t3337 * t1450 + 0.496875e-1 * t1449 * t3356 * t1466 - 0.19875e0 * t4828 * t3356 * t1450 - 0.6625e-1 * t1413 * t8705 * t448 + 0.33125e-1 * t23708 * t42 + 0.99375e-1 * t23711 * t1450 + 0.99375e-1 * t16074 * t3340 + 0.99375e-1 * t1449 * t23716 - 0.33125e-1 * t4769 * t3356 + 0.33125e-1 * t4218 * t6692 - 0.33125e-1 * t8599 * t1466 + 0.165625e-1 * t6631 * t3337 + 0.33125e-1 * t2481 * t8661 + 0.165625e-1 * t430 * (t23779 + t23819) - 0.165625e-1 * t453 * (t23855 + t23887);
    (t23891,)
}
