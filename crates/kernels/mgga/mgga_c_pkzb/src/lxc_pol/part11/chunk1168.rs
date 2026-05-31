//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1168/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1168<F: Float>(t10405: F, t10408: F, t10411: F, t10454: F, t12227: F, t12431: F, t1413: F, t19467: F, t23711: F, t2507: F, t2513: F, t2528: F, t28684: F, t28721: F, t3337: F, t3340: F, t3356: F, t4218: F, t448: F, t453: F, t459: F, t4772: F, t4828: F, t6634: F, t8599: F, t8607: F, t8661: F, t995: F) -> F {
    let t28759 = -F::cast_from(0.165625e-1_f64) * t453 * (t28684 + t28721) + F::cast_from(0.298125e0_f64) * t4772 * t10454 * t448 - F::cast_from(0.59625e0_f64) * t4828 * t10454 * t459 + F::cast_from(0.496875e-1_f64) * t12431 * t3337 + F::cast_from(0.496875e-1_f64) * t4218 * t8661 - F::cast_from(0.19875e0_f64) * t12227 * t8607 + F::cast_from(0.298125e0_f64) * t23711 * t2513 + F::cast_from(0.298125e0_f64) * t19467 * t10405 + F::cast_from(0.298125e0_f64) * t4772 * t2507 * t3340 - F::cast_from(0.99375e-1_f64) * t6634 * t10408 - F::cast_from(0.99375e-1_f64) * t1413 * t8661 * t995 - F::cast_from(0.99375e-1_f64) * t1413 * t3337 * t2528 - F::cast_from(0.99375e-1_f64) * t6634 * t10411 - F::cast_from(0.99375e-1_f64) * t8599 * t2528 - F::cast_from(0.99375e-1_f64) * t1413 * t2507 * t3356;
    t28759
}
